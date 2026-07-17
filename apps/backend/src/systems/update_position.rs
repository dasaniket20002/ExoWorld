use crate::{
    components::{
        acceleration::Acceleration, entity_id::EntityID, position::Position, velocity::Velocity,
    },
    resources::{config::Config, quadtree::Quadtree, time::Time},
};
use bevy_ecs::{
    query::Changed,
    system::{Query, Res, ResMut},
};

pub fn update_position(
    mut query: Query<(&EntityID, &mut Position, &mut Velocity, &Acceleration)>,
    time: Res<Time>,
    config: Res<Config>,
) {
    query.par_iter_mut().for_each(|(_, mut pos, mut vel, acc)| {
        vel.add_acceleration(&acc, &time.delta());
        pos.add_velocity(&vel, &time.delta());

        if pos.get().x < config.world_range.0.0 {
            pos.get().x = config.world_range.0.0;
            vel.get().x = -vel.get().x;
        }

        if pos.get().x > config.world_range.0.1 {
            pos.get().x = config.world_range.0.1;
            vel.get().x = -vel.get().x;
        }

        if pos.get().y < config.world_range.1.0 {
            pos.get().y = config.world_range.1.0;
            vel.get().y = -vel.get().y;
        }

        if pos.get().y > config.world_range.1.1 {
            pos.get().y = config.world_range.1.1;
            vel.get().y = -vel.get().y;
        }
    });
}

pub fn update_quadtree(
    query: Query<(&EntityID, &Position), Changed<Position>>,
    mut quadtree: ResMut<Quadtree>,
) {
    query.iter().for_each(|(id, pos)| {
        quadtree.insert_or_update(&id.get(), &pos.get());
    });
}
