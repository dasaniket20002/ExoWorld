use crate::systems::{
    accumulate_frame_stats::accumulate_frame_stats, log_stats_system::log_stats_system,
    spawn_entities::spawn_entities, update_position::update_position,
};
use bevy_ecs::{
    schedule::{Schedule, ScheduleLabel, Schedules},
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
    startup_schedule.add_systems((spawn_entities,));

    let mut update_schedule = Schedule::new(Update);
    update_schedule.add_systems((accumulate_frame_stats,));

    let mut fixed_schedule = Schedule::new(FixedUpdate);
    fixed_schedule.add_systems((update_position,));

    let mut log_schedule = Schedule::new(Logging);
    log_schedule.add_systems((log_stats_system,));

    let mut schedules = world.resource_mut::<Schedules>();
    schedules.insert(startup_schedule);
    schedules.insert(update_schedule);
    schedules.insert(fixed_schedule);
    schedules.insert(log_schedule);
}
