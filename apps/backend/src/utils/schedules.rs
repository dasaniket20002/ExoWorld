use bevy_ecs::schedule::Schedule;

use crate::systems::log_stats::log_stats;

pub struct Schedules {
    pub update: Schedule,
    pub fixed_update: Schedule,
    pub logging: Schedule,
}

impl Default for Schedules {
    fn default() -> Self {
        Self {
            update: Schedule::default(),
            fixed_update: Schedule::default(),
            logging: Schedule::default(),
        }
    }
}

impl Schedules {
    pub fn build_schedules() -> Self {
        let update = Schedule::default();
        let fixed_update = Schedule::default();
        let mut logging = Schedule::default();

        logging.add_systems(log_stats);

        Self {
            update,
            fixed_update,
            logging,
        }
    }
}
