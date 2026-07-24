use std::time::Instant;

use crate::{
    components::{
        acceleration::Acceleration, entity_id::EntityID, position::Position, radius::Radius,
        velocity::Velocity,
    },
    resources::config::Config,
};
use bevy_ecs::system::{Commands, Res};
use shared::{poisson_disk::sampler::PoissonDiskSample, utils::rng::random_f32};

pub fn spawn_entities(mut cmd: Commands, config: Res<Config>) {
    let start = Instant::now();

    let entities = (0..config.max_entities)
        .map(|_| PoissonDiskSample {
            position: (0.0, 0.0),
            radius: random_f32(0.5, 1.25),
        })
        .map(|sample| {
            let id = EntityID::default();
            let vel = Velocity::default();
            let acc = Acceleration::new(random_f32(-1.0, 1.0), random_f32(-1.0, 1.0));

            let entity_bundle = (
                id,
                Radius((sample.radius * 100.0) as u8),
                Position::new(sample.position.0, sample.position.1),
                vel,
                acc,
            );

            entity_bundle
        })
        .collect::<Vec<_>>();

    let spawn_count = entities.len();
    cmd.spawn_batch(entities);

    println!(
        "[INFO] Spawnned {} entities in {:.2} ms",
        spawn_count,
        start.elapsed().as_secs_f32() * 1000.0
    );
}
