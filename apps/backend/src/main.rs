use crate::{
    resources::{config::Config, stats::Stats, time::UpdateTime},
    utils::{runner::Runner, schedules::Schedules},
};
use bevy_ecs::world::World;

mod components;
mod resources;
mod systems;
mod utils;

fn main() {
    let mut world = World::new();

    world.insert_resource(UpdateTime::default());
    world.insert_resource(Stats::default());
    world.insert_resource(Config::default());

    let schedules = Schedules::build_schedules();

    Runner::new(world, schedules).run();
}
