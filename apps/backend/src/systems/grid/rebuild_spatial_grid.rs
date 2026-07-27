use bevy_ecs::{
    entity::Entity,
    system::{Query, Res, ResMut},
};

use crate::{
    components::{position::Position, radius::Radius, spatial_slot::SpatialSlot},
    resources::spatial_hash_grid::{
        csr_grid::CsrGrid, slot_allocator::SlotAllocator, spatial_grid::SpatialGrid,
    },
    utils::etc::par_write_ptr::ParallelWritePtr,
};

pub fn rebuild_spatial_grid(
    query: Query<(Entity, &Position, &Radius, &SpatialSlot)>,
    allocator: Res<SlotAllocator>,
    mut grid: ResMut<SpatialGrid>,
) {
    let cap = allocator.capacity();
    grid.pos_x.resize(cap, 0.0);
    grid.pos_y.resize(cap, 0.0);
    grid.radius.resize(cap, 0.0);
    grid.entity.resize(cap, Entity::PLACEHOLDER);
    grid.alive.fill(false);
    grid.alive.resize(cap, false);

    // --- Phase 0: race-free parallel scatter extraction ---
    let px = ParallelWritePtr::new(grid.pos_x.as_mut_ptr());
    let py = ParallelWritePtr::new(grid.pos_y.as_mut_ptr());
    let pr = ParallelWritePtr::new(grid.radius.as_mut_ptr());
    let pe = ParallelWritePtr::new(grid.entity.as_mut_ptr());
    let pa = ParallelWritePtr::new(grid.alive.as_mut_ptr());

    query.par_iter().for_each(|(e, pos, radius, slot)| {
        let i = slot.0 as usize;
        unsafe {
            px.write(i, pos.get().x);
            py.write(i, pos.get().y);
            pr.write(i, radius.0 as f32 / 100.0);
            pe.write(i, e);
            pa.write(i, true);
        }
    });

    // --- Pull out everything the key closures need as plain locals ---
    // No closure below touches `grid` directly, so `grid` stays free to
    // be mutably reassigned afterwards.
    let origin_x = grid.origin_x;
    let origin_y = grid.origin_y;
    let coarse_inv = grid.coarse_inv;
    let coarse_dims = grid.coarse_dims;
    let fine_cell_size = grid.fine_cell_size;
    let fine_table_bits = grid.fine_table_bits;

    // Slices borrowed from `grid`'s Vecs directly (not through `grid` itself
    // inside the closure body) — these are plain `&[T]`, ordinary borrows,
    // fully disjoint from the later `grid.coarse = ...` assignment because
    // we won't hold onto them past this point (NLL ends their lifetime at
    // last use, i.e. inside `build_csr`).
    let alive: &[bool] = &grid.alive;
    let pos_x: &[f32] = &grid.pos_x;
    let pos_y: &[f32] = &grid.pos_y;

    let coarse_key = move |i: usize| -> u32 {
        if !alive[i] {
            return (coarse_dims.0 * coarse_dims.1) as u32;
        }
        let cx = (((pos_x[i] - origin_x) * coarse_inv) as i32).clamp(0, coarse_dims.0 - 1);
        let cy = (((pos_y[i] - origin_y) * coarse_inv) as i32).clamp(0, coarse_dims.1 - 1);
        (cy as u32) * coarse_dims.0 as u32 + cx as u32
    };

    let table = 1usize << fine_table_bits;

    let fine_key = move |i: usize| -> u32 {
        if !alive[i] {
            return table as u32;
        }
        let cx = ((pos_x[i] - origin_x) / fine_cell_size).floor() as i32;
        let cy = ((pos_y[i] - origin_y) / fine_cell_size).floor() as i32;
        let hx = (cx as u32).wrapping_mul(0x8da6b343);
        let hy = (cy as u32).wrapping_mul(0xd8163841);
        (hx ^ hy) & (table as u32 - 1)
    };

    let coarse = CsrGrid::build_csr(
        cap,
        (coarse_dims.0 * coarse_dims.1) as usize + 1,
        coarse_key,
    );
    let fine = CsrGrid::build_csr(cap, table + 1, fine_key);

    // Both closures (and their borrows of grid.alive/pos_x/pos_y) are done
    // by now — only here do we take a mutable borrow of `grid`.
    grid.coarse = coarse;
    grid.fine = fine;
}
