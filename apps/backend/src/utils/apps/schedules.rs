use crate::systems::{
    entities::{
        spawn::{insert_to_grid::insert_to_grid, spawn_entities::spawn_entities},
        update::{rebuild_grid::rebuild_grid, update_position::update_position},
    },
    stats::{calculate_stats::calculate_stats, log_stats_system::log_stats_system},
    time::accumulate_frame_stats::accumulate_frame_stats,
};
use bevy_ecs::{
    schedule::{IntoScheduleConfigs, Schedule, ScheduleLabel, Schedules},
    world::World,
};

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Startup;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Update;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FixedUpdate;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Logging;

pub fn register_schedules(world: &mut World) {
    world.init_resource::<Schedules>();

    let mut startup_schedule = Schedule::new(Startup);
    startup_schedule.add_systems((spawn_entities, insert_to_grid).chain());

    let mut update_schedule = Schedule::new(Update);
    update_schedule.add_systems((accumulate_frame_stats,));

    let mut fixed_schedule = Schedule::new(FixedUpdate);
    fixed_schedule.add_systems((update_position, rebuild_grid).chain());

    let mut log_schedule = Schedule::new(Logging);
    log_schedule.add_systems((calculate_stats, (log_stats_system,)).chain());

    let mut schedules = world.resource_mut::<Schedules>();
    schedules.insert(startup_schedule);
    schedules.insert(update_schedule);
    schedules.insert(fixed_schedule);
    schedules.insert(log_schedule);
}
