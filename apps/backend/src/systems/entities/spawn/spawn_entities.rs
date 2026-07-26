use crate::{
    components::{
        acceleration::Acceleration, entity_id::EntityID, position::Position, radius::Radius,
        velocity::Velocity,
    },
    resources::config::Config,
    systems::entities::spawn::poisson_disk::poisson_disk_sampling,
    utils::etc::random_f32,
};
use bevy_ecs::system::{Commands, Res};
use bevy_tasks::{ComputeTaskPool, ParallelSlice, ParallelSliceMut};
use std::time::Instant;

pub fn spawn_entities(mut cmd: Commands, config: Res<Config>) {
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

    // Build bundles in parallel; spawning into `Commands` itself must stay
    // single-threaded (Bevy's World requires exclusive access), so we do the
    // CPU-heavy bundle construction in parallel then spawn once at the end.
    let bundles = samples
        .par_splat_map(pool, None, |_start_idx, chunk| {
            chunk
                .iter()
                .map(|sample| {
                    let id = EntityID::default();
                    let vel = Velocity::default();
                    let acc = Acceleration::new(random_f32(-1.0, 1.0), random_f32(-1.0, 1.0));

                    (
                        id,
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
