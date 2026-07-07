use bevy_ecs::schedule::Schedule;

use crate::systems::{log_stats::log_stats, movement::movement, spawn_entities::spawn_entities};

pub struct Schedules {
    pub startup: Schedule,
    pub update: Schedule,
    pub fixed_update: Schedule,
    pub logging: Schedule,
}

impl Default for Schedules {
    fn default() -> Self {
        Self {
            startup: Schedule::default(),
            update: Schedule::default(),
            fixed_update: Schedule::default(),
            logging: Schedule::default(),
        }
    }
}

impl Schedules {
    pub fn build_schedules() -> Self {
        let mut startup = Schedule::default();
        let mut update = Schedule::default();
        let mut fixed_update = Schedule::default();
        let mut logging = Schedule::default();

        startup.add_systems(spawn_entities);
        update.add_systems(movement);
        // fixed_update.add_systems(movement);
        logging.add_systems(log_stats);

        Self {
            startup,
            update,
            fixed_update,
            logging,
        }
    }
}
