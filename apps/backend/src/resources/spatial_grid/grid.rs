use crate::resources::spatial_grid::{cell::Cell, chunk::Chunk};
use bevy_ecs::entity::EntityHashMap;
use bevy_ecs::{entity::Entity, resource::Resource};
use bevy_math::ops::{ceil, floor};
use std::fs::File;
use std::io::Write;

pub type Grid = Vec<Chunk>;

#[derive(Clone, Copy, Debug)]
pub struct SpatialGridParams {
    pub chunk_size: usize,
    pub cell_size: usize,

    pub chunks_per_row: usize,
    pub cells_per_row: usize,
}

#[derive(Resource, Debug)]
pub struct SpatialGrid {
    pub params: SpatialGridParams,
    active: Grid,
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

        // let active = vec![vec![Vec::<Entity>::new(); cells_per_chunk]; total_chunks];
        let active = (0..total_chunks)
            .map(|chunk_id| {
                Chunk::with_cells(
                    chunk_id,
                    (0..cells_per_chunk)
                        .map(|cell_id| Cell::new(cell_id))
                        .collect(),
                )
            })
            .collect();

        Self {
            params: SpatialGridParams {
                chunk_size,
                cell_size,

                chunks_per_row,
                cells_per_row,
            },
            active,
        }
    }

    #[inline]
    pub fn get_active_mut(&mut self) -> &mut Grid {
        self.active.as_mut()
    }

    #[inline]
    pub fn get_chunk_mut(&mut self, chunk_idx: usize) -> &mut Chunk {
        unsafe { self.active.get_unchecked_mut(chunk_idx) }
    }

    #[inline]
    pub fn g_world_to_chunk_id(x: f32, y: f32, params: SpatialGridParams) -> usize {
        let cx = (floor(x / params.chunk_size as f32) as usize).min(params.chunks_per_row - 1);
        let cy = (floor(y / params.chunk_size as f32) as usize).min(params.chunks_per_row - 1);
        cy * params.chunks_per_row + cx
    }

    #[inline]
    pub fn g_world_to_cell_id(x: f32, y: f32, params: SpatialGridParams) -> usize {
        let local_x = x % params.chunk_size as f32;
        let local_y = y % params.chunk_size as f32;
        let cx = (floor(local_x / params.cell_size as f32) as usize).min(params.cells_per_row - 1);
        let cy = (floor(local_y / params.cell_size as f32) as usize).min(params.cells_per_row - 1);
        cy * params.cells_per_row + cx
    }

    #[inline]
    pub fn world_to_chunk_id(&self, x: f32, y: f32) -> usize {
        Self::g_world_to_chunk_id(x, y, self.params)
    }

    #[inline]
    pub fn world_to_cell_id(&self, x: f32, y: f32) -> usize {
        Self::g_world_to_cell_id(x, y, self.params)
    }

    /// Inserts the `entity` at the `chunk` and `cell` location
    ///
    /// Returns: The index of the array at which the entity was pushed
    #[inline]
    pub fn insert_entity_at(&mut self, entity: Entity, chunk_id: usize, cell_id: usize) -> usize {
        let chunk = self.get_chunk_mut(chunk_id);
        let cell = chunk.get_cell_mut(cell_id);

        cell.push_entity(entity)
    }

    // /// Removes an `entity` from the given `chunk`, `cell` and `slot`.
    // /// Replaces the removed item with the last item to reduce reallocation cost
    // ///
    // /// Returns: The `entity` that was swapped to fill in
    // #[inline]
    // pub fn remove_entity_at(
    //     &mut self,
    //     chunk_id: usize,
    //     cell_id: usize,
    //     cell_slot: usize,
    // ) -> Entity {
    //     let chunk = self.get_chunk_mut(chunk_id);
    //     let cell = chunk.get_cell_mut(cell_id);

    //     cell.remove_entity_at(cell_slot)
    // }

    pub fn dump_to_file(
        &self,
        filename: &str,
        positions: &EntityHashMap<(f32, f32, usize, usize, usize)>, // entity → world position
    ) {
        let mut f = File::create(filename).expect("Cannot create debug file");
        writeln!(f, "=== Spatial Grid Debug Dump ===").ok();
        writeln!(f, "Params: {:?}\n", self.params).ok();

        for (chunk_idx, chunk) in self.active.iter().enumerate() {
            writeln!(f, "Chunk #{}  (cells: {})", chunk_idx, chunk.cells.len()).ok();

            for (cell_idx, cell) in chunk.cells.iter().enumerate() {
                if !cell.is_empty() {
                    write!(f, "  Cell #{}: [", cell_idx).ok();
                    for (i, &entity) in cell.entities().iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ").ok();
                        }
                        let pos_str = match positions.get(&entity) {
                            Some((x, y, _, _, _)) => format!("({}, {})", x, y),
                            None => "no_pos".to_string(),
                        };
                        let loc_str = match positions.get(&entity) {
                            Some((_, _, ch, cl, sl)) => format!("({}, {}, {})", ch, cl, sl),
                            None => "no_loc".to_string(),
                        };
                        write!(f, "{} {} {}", entity, pos_str, loc_str).ok();
                    }
                    writeln!(f, "]").ok();
                }
            }

            // Mutations printing remains the same, but now removals might include entity.
            // You already updated removals to include entity? In your code, it seems like
            // removals now contain (entity, cell, slot)? I'll assume it does.
            // Just adjust formatting accordingly.
            if !chunk.mutations.removals.is_empty() {
                writeln!(f, "    REMOVALS (entity, cell, slot): ").ok();
                for &(entity, cell, slot) in chunk.mutations.removals.iter() {
                    let pos_str = match positions.get(&entity) {
                        Some((x, y, _, _, _)) => format!("({}, {})", x, y),
                        None => "no_pos".to_string(),
                    };
                    write!(f, "      ({} {}, {}, {}) ", entity, pos_str, cell, slot).ok();
                }
                writeln!(f).ok();
            }
            if !chunk.mutations.insertions.is_empty() {
                writeln!(f, "    INSERTIONS (entity, cell): ").ok();
                for &(entity, cell) in chunk.mutations.insertions.iter() {
                    let pos_str = match positions.get(&entity) {
                        Some((x, y, _, _, _)) => format!("({}, {})", x, y),
                        None => "no_pos".to_string(),
                    };
                    write!(f, "      ({} {}, {}) ", entity, pos_str, cell).ok();
                }
                writeln!(f).ok();
            }
            writeln!(f).ok();
        }
    }
}
