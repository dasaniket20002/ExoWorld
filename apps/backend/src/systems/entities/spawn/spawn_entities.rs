use crate::{
    components::{
        acceleration::Acceleration, entity_id::EntityID, position::Position, radius::Radius,
        spatial_slot::SpatialSlot, velocity::Velocity,
    },
    resources::{config::Config, spatial_hash_grid::slot_allocator::SlotAllocator},
    systems::entities::spawn::poisson_disk::poisson_disk_sampling,
    utils::etc::random_f32,
};
use bevy_ecs::system::{Commands, Res, ResMut};
use bevy_tasks::{ComputeTaskPool, ParallelSlice, ParallelSliceMut};
use std::time::Instant;

pub fn spawn_entities(
    mut cmd: Commands,
    config: Res<Config>,
    mut allocator: ResMut<SlotAllocator>,
) {
    let start = Instant::now();
    let pool = ComputeTaskPool::get();

    // Fill radii in parallel — par_splat_map_mut auto-chunks across the pool.
    let mut radii = vec![0.0_f32; config.max_entities as usize];
    radii.par_splat_map_mut(pool, None, |_start_idx, chunk| {
        for r in chunk.iter_mut() {
            *r = random_f32(0.5, 1.25);
        }
    });
    let samples = poisson_disk_sampling(&radii, &config.world_bounds, 30);

    let spawn_count = samples.len();
    if spawn_count == 0 {
        return; // nothing to spawn
    }

    let mut pre_alloc_slots: Vec<u32> = Vec::with_capacity(spawn_count);
    for _ in 0..spawn_count {
        pre_alloc_slots.push(allocator.alloc());
    }

    // Build bundles in parallel; spawning into `Commands` itself must stay
    // single-threaded (Bevy's World requires exclusive access), so we do the
    // CPU-heavy bundle construction in parallel then spawn once at the end.
    let bundles = samples
        .par_splat_map(pool, None, |start_idx, chunk| {
            chunk
                .iter()
                .enumerate()
                .map(|(local_idx, sample)| {
                    let global_idx = start_idx + local_idx;
                    let slot = pre_alloc_slots[global_idx];

                    let slot = SpatialSlot(slot);
                    let id = EntityID::default();
                    let vel = Velocity::default();
                    let acc = Acceleration::new(random_f32(-1.0, 1.0), random_f32(-1.0, 1.0));

                    (
                        id,
                        slot,
                        Radius((sample.radius * 100.0) as u8),
                        Position::new(sample.position.0, sample.position.1),
                        vel,
                        acc,
                    )
                })
                .collect::<Vec<_>>()
        })
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    let spawn_count = bundles.len();
    cmd.spawn_batch(bundles);

    println!(
        "[INFO] Spawned {} entities in {:.2} ms",
        spawn_count,
        start.elapsed().as_secs_f32() * 1000.0
    );
}
