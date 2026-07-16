use bevy_ecs::resource::Resource;
use std::time::Instant;

#[derive(Resource)]
pub struct Time {
    pub frame_start: Instant,
    delta: f32,
    fixed_delta: f32,

    pub _running_fixed_update: bool,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            frame_start: Instant::now(),
            delta: 0.0,
            fixed_delta: 0.0,

            _running_fixed_update: false,
        }
    }
}

impl Time {
    pub fn set_update_delta(&mut self, delta: f32) {
        self.delta = delta;
    }

    pub fn set_fixed_update_delta(&mut self, delta: f32) {
        self.fixed_delta = delta;
    }

    pub fn delta(&self) -> f32 {
        if self._running_fixed_update {
            self.fixed_delta
        } else {
            self.delta
        }
    }
}

#[derive(Resource, Default)]
pub struct FixedUpdateAccumulator {
    pub remainder: f32,
}

#[derive(Resource, Default)]
pub struct LoggingAccumulator {
    pub remainder: f32,
}
