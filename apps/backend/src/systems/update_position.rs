use crate::{
    components::{acceleration::Acceleration, position::Position, velocity::Velocity},
    resources::time::Time,
};
use bevy_ecs::system::{Query, Res};

pub fn update_position(
    mut query: Query<(&mut Position, &mut Velocity, &Acceleration)>,
    time: Res<Time>,
) {
    for (mut pos, mut vel, acc) in &mut query {
        vel.add_acceleration(&acc, &time.delta());
        pos.add_velocity(&vel, &time.delta());
    }
    // query.par_iter_mut().for_each(|(mut pos, mut vel, acc)| {
    //     vel.add_acceleration(&acc, &time.delta());
    //     pos.add_velocity(&vel, &time.delta());
    // });
}
