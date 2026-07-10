mod components;
mod resources;
mod systems;
mod utils;

use crate::{
    resources::{
        config::Config,
        engine_stats::EngineStats,
        time::{FixedUpdateAccumulator, LoggingAccumulator, Time},
    },
    utils::{runner::Runner, schedules::register_schedules},
};
use bevy_ecs::world::World;

fn main() {
    let mut world = World::new();

    println!("[INFO] World initialized");

    world.init_resource::<Config>();
    world.init_resource::<Time>();
    world.init_resource::<FixedUpdateAccumulator>();
    world.init_resource::<LoggingAccumulator>();
    world.init_resource::<EngineStats>();

    register_schedules(&mut world);

    println!("[INFO] World resources and schedules inserted");

    Runner::new(world).run();
}
