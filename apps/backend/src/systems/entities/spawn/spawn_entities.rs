use crate::{
    components::{
        acceleration::Acceleration, facing::Facing, position::Position, radius::Radius,
        velocity::Velocity,
    },
    resources::config::Config,
    systems::entities::spawn::poisson_disk::poisson_disk_sampling,
    utils::etc::random_f32,
};
use bevy_ecs::system::{Commands, Res};
use indicatif::ProgressBar;
use std::time::Instant;

pub fn spawn_entities(mut cmd: Commands, config: Res<Config>) {
    let start = Instant::now();
    let pb = ProgressBar::new(config.max_entities as u64).with_elapsed(start.elapsed());

    let radii = (0..config.max_entities)
        .map(|_| random_f32(0.5, 1.25))
        .collect::<Vec<_>>();

    let samples = poisson_disk_sampling(&radii, config.world_size, 30, &pb);
    let spawn_count = samples.len();

    let bundles = samples.into_iter().map(move |sample| {
        let vel = Velocity::default();
        let acc = Acceleration::new(random_f32(-1.0, 1.0), random_f32(-1.0, 1.0));
        let rad = Radius((sample.radius * 100.0) as u8);
        let pos = Position::new(sample.position.0, sample.position.1);
        let fac = Facing::default();

        (rad, pos, vel, acc, fac)
    });

    cmd.spawn_batch(bundles);
    pb.finish_and_clear();

    println!(
        "[INFO] Spawned {} entities in {:.2} ms",
        spawn_count,
        start.elapsed().as_secs_f32() * 1000.0,
    );
}
