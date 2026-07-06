use std::time::Duration;

use bevy_ecs::resource::Resource;

#[derive(Resource)]
pub struct Config {
    pub expected_tps: u16,
    pub fixed_update_interval: Duration,
    pub logging_interval: Duration,

    pub max_fixed_updates_per_frame: u16,
}

impl Default for Config {
    fn default() -> Self {
        let tps = 20;
        Self {
            expected_tps: tps,
            fixed_update_interval: Duration::from_secs_f64(1.0 / tps as f64),
            logging_interval: Duration::from_secs(2),
            max_fixed_updates_per_frame: 5,
        }
    }
}
