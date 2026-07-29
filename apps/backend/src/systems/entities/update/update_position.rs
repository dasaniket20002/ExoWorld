use crate::{
    components::{
        acceleration::Acceleration, entity_id::EntityID, position::Position, velocity::Velocity,
    },
    resources::{config::Config, time::time::Time},
};
use bevy_ecs::system::{Query, Res};

pub fn update_position(
    mut query: Query<(&EntityID, &mut Position, &mut Velocity, &Acceleration)>,
    time: Res<Time>,
    config: Res<Config>,
) {
    let (minx, miny) = config.world_bounds.0;
    let (maxx, maxy) = config.world_bounds.1;

    query.par_iter_mut().for_each(|(_, mut pos, mut vel, acc)| {
        vel.add_acceleration(&acc, &time.delta());
        pos.add_velocity(&vel, &time.delta());

        if pos.0 < minx {
            pos.0 = minx;
            vel.0 = -vel.0;
        }

        if pos.0 > maxx {
            pos.0 = maxx;
            vel.0 = -vel.0;
        }

        if pos.1 < miny {
            pos.1 = miny;
            vel.1 = -vel.1;
        }

        if pos.1 > maxy {
            pos.1 = maxy;
            vel.1 = -vel.1;
        }
    });
}
