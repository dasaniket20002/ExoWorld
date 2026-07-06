use bevy_ecs::resource::Resource;
use std::time::Duration;

#[derive(Resource)]
pub struct Stats {
    pub frame_time: Duration,

    pub current_tps: u16,
    pub average_tps: f32,

    pub min_tps: u16,
    pub max_tps: u16,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            frame_time: Duration::ZERO,
            current_tps: 0,
            average_tps: 0.0,
            min_tps: 0,
            max_tps: 0,
        }
    }
}
