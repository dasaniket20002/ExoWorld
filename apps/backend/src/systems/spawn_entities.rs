use crate::{
    components::{
        acceleration::Acceleration, entity_id::EntityID, position::Position, velocity::Velocity,
    },
    resources::{config::Config, quadtree::Quadtree},
};
use bevy_ecs::system::{Commands, Res, ResMut};

pub fn spawn_entities(mut cmd: Commands, config: Res<Config>, mut quadtree: ResMut<Quadtree>) {
    let mut entities = vec![];

    for _ in 0..config.max_entities {
        let id = EntityID::default();
        let pos = Position::new(
            (fastrand::f32() * (config.world_range.0.1 - config.world_range.0.0 + 1.0))
                + config.world_range.0.0,
            (fastrand::f32() * (config.world_range.1.1 - config.world_range.1.0 + 1.0))
                + config.world_range.1.0,
        );
        let vel = Velocity::default();
        let acc = Acceleration::new(
            (fastrand::f32() * (1.0 - -1.0 + 1.0)) + -1.0,
            (fastrand::f32() * (1.0 - -1.0 + 1.0)) + -1.0,
        );

        quadtree.insert_or_update(&id.get(), &pos.get());
        entities.push((id, pos, vel, acc));
    }

    cmd.spawn_batch(entities);

    println!("[INFO] Spawnned {} entities", config.max_entities);
}
