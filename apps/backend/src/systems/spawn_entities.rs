use crate::{
    components::{acceleration::Acceleration, position::Position, velocity::Velocity},
    resources::config::Config,
};
use bevy_ecs::system::{Commands, Res};

pub fn spawn_entities(mut cmd: Commands, config: Res<Config>) {
    let mut entities = vec![];

    let range = (
        -(config.max_entities as f32) / 2.0,
        (config.max_entities as f32) / 2.0,
    );

    for _ in 0..config.max_entities {
        entities.push((
            Position::new(
                (fastrand::f32() * (range.1 - range.0 + 1.0)) + range.0,
                0.0,
                (fastrand::f32() * (range.1 - range.0 + 1.0)) + range.0,
            ),
            Velocity::default(),
            Acceleration::new(
                (fastrand::f32() * (1.0 - -1.0 + 1.0)) + -1.0,
                0.0,
                (fastrand::f32() * (1.0 - -1.0 + 1.0)) + -1.0,
            ),
        ));
    }

    cmd.spawn_batch(entities);

    println!("[INFO] Spawnned {} entities", config.max_entities);
}
