use crate::{
    components::{acceleration::Acceleration, position::Position, velocity::Velocity},
    resources::config::Config,
};
use bevy_ecs::system::{Commands, Res};
use rand::random_range;

pub fn spawn_entities(mut cmd: Commands, config: Res<Config>) {
    let mut entities = vec![];

    let range = (
        -(config.max_entities as f32) / 2.0,
        (config.max_entities as f32) / 2.0,
    );

    for _ in 0..config.max_entities {
        entities.push((
            Position::new(
                random_range(range.0..range.1),
                random_range(range.0..range.1),
                0.0,
            ),
            Velocity::default(),
            Acceleration::new(random_range(-1.0..1.0), random_range(-1.0..1.0), 0.0),
        ));
    }

    cmd.spawn_batch(entities);
    
    println!("Spawnned {} entities", config.max_entities);
}
