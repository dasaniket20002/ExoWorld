use crate::{
    components::{acceleration::Acceleration, position::Position, velocity::Velocity},
    resources::time::Time,
};
use bevy_ecs::{
    batching::BatchingStrategy,
    system::{Query, Res},
};

pub fn movement(mut query: Query<(&mut Position, &mut Velocity, &Acceleration)>, time: Res<Time>) {
    // for (mut pos, mut vel, acc) in &mut query {
    //     vel.add_acceleration(&acc, &time.delta());
    //     pos.add_velocity(&vel, &time.delta());
    // }

    query
        .par_iter_mut()
        .batching_strategy(BatchingStrategy::fixed(100000))
        .for_each(|(mut pos, mut vel, acc)| {
            vel.add_acceleration(&acc, &time.delta());
            pos.add_velocity(&vel, &time.delta());
        });
}
