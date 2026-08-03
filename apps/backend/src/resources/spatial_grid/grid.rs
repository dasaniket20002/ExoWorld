use crate::components::position::Position;
use bevy_ecs::{entity::Entity, resource::Resource};
use bevy_math::ops::{ceil, floor};

pub type Cell = Vec<Entity>;
pub type Chunk = Vec<Cell>;

#[derive(Clone, Copy)]
pub struct SpatialGridParams {
    world_size: usize,

    chunk_size: usize,
    cell_size: usize,

    chunks_per_row: usize,
    total_chunks: usize,

    cells_per_row: usize,
    cells_per_chunk: usize,
}

#[derive(Resource)]
pub struct SpatialGrid {
    params: SpatialGridParams,
    pub active: Vec<Chunk>,
}

impl SpatialGrid {
    pub fn new(world_size: usize, chunk_size: usize, cell_size: usize) -> Self {
        let chunks_per_row = ceil(world_size as f32 / chunk_size as f32) as usize;
        let total_chunks = chunks_per_row * chunks_per_row;

        let cells_per_row = ceil(chunk_size as f32 / cell_size as f32) as usize;
        let cells_per_chunk = cells_per_row * cells_per_row;

        println!(
            "[INFO] Grid initialized with {} chunks and {} cells per chunk",
            total_chunks, cells_per_chunk
        );

        let active = vec![vec![Vec::<Entity>::new(); cells_per_chunk]; total_chunks];

        Self {
            params: SpatialGridParams {
                world_size,

                chunk_size,
                cell_size,

                chunks_per_row,
                total_chunks,

                cells_per_row,
                cells_per_chunk,
            },
            active,
        }
    }

    #[inline]
    pub fn g_world_to_chunk_id(x: f32, y: f32, params: SpatialGridParams) -> usize {
        let cx = floor(x / params.chunk_size as f32) as usize;
        let cy = floor(y / params.chunk_size as f32) as usize;

        cy * params.chunks_per_row + cx
    }

    #[inline]
    pub fn world_to_chunk_id(&self, x: f32, y: f32) -> usize {
        Self::g_world_to_chunk_id(x, y, self.params)
    }

    #[inline]
    pub fn g_world_to_cell_id(x: f32, y: f32, params: SpatialGridParams) -> usize {
        let cx = floor(x % params.chunk_size as f32) as usize / params.cell_size;
        let cy = floor(y % params.chunk_size as f32) as usize / params.cell_size;

        cy * params.cells_per_row + cx
    }

    #[inline]
    pub fn world_to_cell_id(&self, x: f32, y: f32) -> usize {
        Self::g_world_to_cell_id(x, y, self.params)
    }

    pub fn params(&self) -> SpatialGridParams {
        self.params
    }

    #[inline]
    pub fn insert_entity_at(&mut self, entity: Entity, chunk_id: usize, cell_id: usize) {
        unsafe {
            let chunk = self.active.get_unchecked_mut(chunk_id);
            let cell = chunk.get_unchecked_mut(cell_id);

            cell.push(entity);
        }
    }

    #[inline]
    pub fn insert_entity(&mut self, entity: Entity, position: &Position) {
        let chunk_id = self.world_to_chunk_id(position.0, position.1);
        let cell_id = self.world_to_cell_id(position.0, position.1);

        self.insert_entity_at(entity, chunk_id, cell_id);
    }

    // /// Every entity that could plausibly be within `radius` of `(x, y)`.
    // /// This always reads the frozen `active` snapshot, so it's safe to
    // /// call from any system, on any tick, even mid-rebuild.
    // pub fn query_radius(&self, x: f32, y: f32, radius: f32) -> Vec<Entity> {
    //     let (sx, sy) = Self::g_world_to_grid_id(x, y, self.world_size);

    //     // How many cells does the search radius actually span? Entities
    //     // near a cell edge can have neighbors in adjacent cells, so we
    //     // scan a small window around the center cell rather than just
    //     // the single cell the point falls in.
    //     let cell_span = ceil(radius / self.cell_size as f32) as i32;
    //     let base_cx = floor(sx / self.cell_size as f32) as i32;
    //     let base_cy = floor(sy / self.cell_size as f32) as i32;

    //     let mut out = Vec::new();
    //     for dy in -cell_span..=cell_span {
    //         for dx in -cell_span..=cell_span {
    //             let cx = base_cx + dx;
    //             let cy = base_cy + dy;
    //             if cx < 0
    //                 || cy < 0
    //                 || cx >= self.cells_per_chunk as i32
    //                 || cy >= self.cells_per_chunk as i32
    //             {
    //                 continue;
    //             }

    //             // Re-derive which chunk/cell this absolute cell coordinate
    //             // lands in.
    //             let chunk_cx = cx as u32 / self.cells_per_chunk;
    //             let chunk_cy = cy as u32 / self.cells_per_chunk;
    //             if chunk_cx >= self.chunk_count.0 || chunk_cy >= self.chunk_count.1 {
    //                 continue;
    //             }
    //             let chunk_id = (chunk_cy * self.chunk_count.0 + chunk_cx) as usize;
    //             let local_cx = cx as u32 % self.cells_per_chunk;
    //             let local_cy = cy as u32 % self.cells_per_chunk;
    //             let cell_id = (local_cy * self.cells_per_chunk + local_cx) as usize;

    //             let Some(chunk) = self.active.get(chunk_id) else {
    //                 continue;
    //             };
    //             let Some(cell) = chunk.0.get(cell_id) else {
    //                 continue;
    //             };

    //             out.extend(cell.0.iter());
    //         }
    //     }
    //     out
    // }
}
