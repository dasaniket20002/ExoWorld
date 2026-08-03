mod components;
mod resources;
mod systems;
mod utils;

use crate::{
    resources::{
        config::Config,
        engine_stats::EngineStats,
        spatial_grid::grid::SpatialGrid,
        time::{
            fixed_accumulator::FixedUpdateAccumulator, logging_accumulator::LoggingAccumulator,
            time::Time,
        },
    },
    utils::apps::{runner::Runner, schedules::register_schedules},
};
use bevy_ecs::world::World;
use bevy_tasks::{ComputeTaskPool, TaskPool};

fn main() {
    ComputeTaskPool::get_or_init(TaskPool::default);
    let mut world = World::new();

    println!("[INFO] World initialized");

    world.init_resource::<Config>();
    world.init_resource::<Time>();
    world.init_resource::<FixedUpdateAccumulator>();
    world.init_resource::<LoggingAccumulator>();
    world.init_resource::<EngineStats>();

    {
        let cfg = world.resource::<Config>();
        world.insert_resource::<SpatialGrid>(SpatialGrid::new(
            cfg.world_size,
            cfg.chunk_size,
            cfg.cell_size,
        ));
    }

    register_schedules(&mut world);

    println!("[INFO] World resources and schedules inserted");

    Runner::new(world).run();
}
