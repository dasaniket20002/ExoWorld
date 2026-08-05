use crate::{
    components::{grid_location::GridLocation, position::Position},
    resources::spatial_grid::grid::SpatialGrid,
};
use bevy_ecs::{
    entity::{Entity, EntityHashMap},
    system::{Query, ResMut},
};
use bevy_tasks::{ComputeTaskPool, ParallelSliceMut};
use bevy_utils::Parallel;
// use std::ops::AddAssign;
use std::time::Instant;

pub fn rebuild_grid(
    mut query: Query<(Entity, &Position, &mut GridLocation)>,
    mut grid: ResMut<SpatialGrid>,
    // mut itr: bevy_ecs::system::Local<usize>,
) {
    let start = Instant::now();
    // let mut changed_pos = 0_u32;
    // let mut changed_grid = 0_u32;

    let task_pool = ComputeTaskPool::get();
    let mut parallel_slot_mutations: Parallel<
        EntityHashMap<(
            usize, /* chunk_id */
            usize, /* cell_id */
            usize, /* cell_slot */
        )>,
    > = Parallel::default();

    // detect moves and fill chunk mutation buffers
    query.iter().for_each(|(entity, pos, _loc)| {
        if !pos.is_changed() {
            return;
        }

        // changed_pos += 1;

        let new_chunk = grid.world_to_chunk_id(pos.0, pos.1);
        let new_cell = grid.world_to_cell_id(pos.0, pos.1);

        if new_chunk == _loc.chunk_id && new_cell == _loc.cell_id {
            return;
        }

        // Removal (old location)
        grid.get_chunk_mut(_loc.chunk_id)
            .queue_removal(entity, _loc.cell_id, _loc.cell_slot);

        // Insertion (new location)
        grid.get_chunk_mut(new_chunk)
            .queue_insertion(entity, new_cell);
    });

    // {
    //     let positions = query
    //         .iter()
    //         .map(|(e, p, l)| (e, (p.0, p.1, l.chunk_id, l.cell_id, l.cell_slot)))
    //         .collect();
    //     grid.dump_to_file(&format!("frames/{}_grid.txt", *itr + 1), &positions);
    //     itr.add_assign(1);
    // }

    // parallel grid mutation
    grid.get_active_mut()
        .par_splat_map_mut(task_pool, None, |_range, chunks| {
            let mut local_mutations = parallel_slot_mutations.borrow_local_mut();
            // local_mutations.clear();

            for chunk in chunks {
                chunk.process_removals(&mut local_mutations);
                chunk.process_insertions(&mut local_mutations);

                // Clear for next use
                chunk.clear_mutations();
            }
        });

    // update the grid locations of the moved entities
    parallel_slot_mutations.iter_mut().for_each(|mutations| {
        mutations
            .iter()
            .for_each(|(entity, (chunk_id, cell_id, slot))| {
                let (_, _, mut location) = query.get_mut(*entity).unwrap();

                location.chunk_id = *chunk_id;
                location.cell_id = *cell_id;
                location.cell_slot = *slot;

                // changed_grid += 1;
            });
    });

    println!(
        "rebuild_grid completed in {:.2} ms",
        start.elapsed().as_secs_f32() * 1000.0
    );
    // println!(
    //     "positions: {}, grid locations: {}",
    //     changed_pos, changed_grid
    // );
}
