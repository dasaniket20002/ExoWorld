use crate::resources::{config::Config, spatial_hash_grid::csr_grid::CsrGrid};
use bevy_ecs::{entity::Entity, resource::Resource};

#[derive(Resource, Default)]
pub struct SpatialGrid {
    pub origin_x: f32,
    pub origin_y: f32,

    // ---- coarse (dense array, range queries) ----
    pub coarse_cell_size: f32,
    pub coarse_inv: f32,
    pub coarse_dims: (i32, i32),
    pub coarse: CsrGrid,

    // ---- fine (hashed, collision queries) ----
    pub fine_cell_size: f32,
    pub fine_inv: f32,
    pub fine_table_bits: u32, // table_size = 1 << bits
    pub fine: CsrGrid,

    // ---- shared SoA snapshot, indexed by SpatialSlot ----
    pub pos_x: Vec<f32>,
    pub pos_y: Vec<f32>,
    pub radius: Vec<f32>,
    pub entity: Vec<Entity>,
    pub alive: Vec<bool>,
}

impl SpatialGrid {
    pub fn new(config: &Config, coarse_cell_size: f32, fine_cell_size: f32) -> Self {
        let (min_x, min_y) = config.world_bounds.0;
        let (max_x, max_y) = config.world_bounds.1;

        let world_w = max_x - min_x;
        let world_h = max_y - min_y;

        assert!(coarse_cell_size > 0.0, "coarse_cell_size must be > 0");
        assert!(fine_cell_size > 0.0, "fine_cell_size must be > 0");

        // .ceil().max(1.0) guarantees at least 1 cell per axis even for
        // tiny worlds or oversized cell sizes — this is what prevents the
        // coarse_dims == 0 bug you just hit.
        let coarse_dims = (
            (world_w / coarse_cell_size).ceil().max(1.0) as i32,
            (world_h / coarse_cell_size).ceil().max(1.0) as i32,
        );

        // Pick a fine hash table size large enough to keep load factor low.
        // Round up to next power of two so `& (table - 1)` masking works.
        let expected_entities = config.max_entities.max(1);
        let target_buckets = (expected_entities as f64 * 1.5) as u64;
        let fine_table_bits = 64 - (target_buckets.max(1) - 1).leading_zeros().min(63);

        Self {
            origin_x: min_x,
            origin_y: min_y,

            coarse_cell_size,
            coarse_inv: 1.0 / coarse_cell_size,
            coarse_dims,
            coarse: CsrGrid {
                cell_start: vec![0; (coarse_dims.0 * coarse_dims.1) as usize + 2],
                cell_items: Vec::new(),
            },

            fine_cell_size,
            fine_inv: 1.0 / fine_cell_size,
            fine_table_bits,
            fine: CsrGrid {
                cell_start: vec![0; (1usize << fine_table_bits) + 2],
                cell_items: Vec::new(),
            },

            pos_x: Vec::new(),
            pos_y: Vec::new(),
            radius: Vec::new(),
            entity: Vec::new(),
            alive: Vec::new(),
        }
    }

    #[inline]
    fn coarse_cell(&self, x: f32, y: f32) -> (i32, i32) {
        let fx = ((x - self.origin_x) * self.coarse_inv).floor();
        let fy = ((y - self.origin_y) * self.coarse_inv).floor();
        let cx = (fx as i32).clamp(0, self.coarse_dims.0 - 1);
        let cy = (fy as i32).clamp(0, self.coarse_dims.1 - 1);
        (cx, cy)
    }

    pub fn query_aabb(
        &self,
        min: (f32, f32),
        max: (f32, f32),
        mut visit: impl FnMut(Entity, usize),
    ) {
        let (cx0, cy0) = self.coarse_cell(min.0, min.1);
        let (cx1, cy1) = self.coarse_cell(max.0, max.1);
        let dx = self.coarse_dims.0 as u32;

        for cy in cy0..=cy1 {
            let row = cy as u32 * dx;
            let start = self.coarse.cell_start[(row + cx0 as u32) as usize];
            let end = self.coarse.cell_start[(row + cx1 as u32 + 1) as usize];
            for &slot in &self.coarse.cell_items[start as usize..end as usize] {
                visit(self.entity[slot as usize], slot as usize);
            }
        }
    }

    pub fn query_radius(&self, cx: f32, cy: f32, r: f32, mut visit: impl FnMut(Entity, usize)) {
        self.query_aabb((cx - r, cy - r), (cx + r, cy + r), |e, s| {
            let (dx, dy) = (self.pos_x[s] - cx, self.pos_y[s] - cy);
            if dx * dx + dy * dy <= r * r {
                visit(e, s);
            }
        });
    }
}
