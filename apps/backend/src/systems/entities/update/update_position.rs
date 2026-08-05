use crate::{
    components::{acceleration::Acceleration, position::Position, velocity::Velocity},
    resources::{config::Config, time::time::Time},
};
use bevy_ecs::system::{Query, Res};

pub fn update_position(
    mut query: Query<(&mut Position, &mut Velocity, &Acceleration)>,
    time: Res<Time>,
    config: Res<Config>,
) {
    let world_size = config.world_size as f32;

    query.par_iter_mut().for_each(|(mut pos, mut vel, acc)| {
        if acc.nil() {
            return;
        }

        vel.note_previous();
        pos.note_previous();

        vel.add_acceleration(&acc, &time.delta());
        pos.add_velocity(&vel, &time.delta());

        if pos.0 < 0.0 {
            pos.0 = 0.0;
            vel.0 = -vel.0;
        }

        if pos.0 > world_size {
            pos.0 = world_size;
            vel.0 = -vel.0;
        }

        if pos.1 < 0.0 {
            pos.1 = 0.0;
            vel.1 = -vel.1;
        }

        if pos.1 > world_size {
            pos.1 = world_size;
            vel.1 = -vel.1;
        }
    });
}
