mod components;
mod resources;
mod systems;
mod utils;

use crate::{
    resources::{
        config::Config,
        engine_stats::EngineStats,
        spatial_hash_grid::{
            collision_pairs::CollisionPairs, slot_allocator::SlotAllocator,
            spatial_grid::SpatialGrid,
        },
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
    world.init_resource::<SlotAllocator>();
    world.init_resource::<CollisionPairs>();

    // coarse cell size: tune for your typical range-query size,
    // e.g. ~1000 units → ~1000×1000 = 1,000,000 coarse cells for a 1e6² world
    let coarse_cell_size = 1000.0;
    // fine cell size: ~2× max entity radius (radius up to 1.25 → diameter 2.5)
    let fine_cell_size = 3.0;
    let cfg = world.resource::<Config>();
    world.insert_resource(SpatialGrid::new(cfg, coarse_cell_size, fine_cell_size));

    register_schedules(&mut world);

    println!("[INFO] World resources and schedules inserted");

    Runner::new(world).run();
}
