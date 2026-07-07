use bevy_ecs::resource::Resource;
use std::time::Duration;

#[derive(Resource)]
pub struct Time {
    _running_fixed_update: bool,

    delta: Duration,
    fixed_delta: Duration,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            _running_fixed_update: false,
            delta: Duration::ZERO,
            fixed_delta: Duration::ZERO,
        }
    }
}

impl Time {
    pub fn running_fixed_update(&mut self, r: bool) {
        self._running_fixed_update = r;
    }

    pub fn set_update_delta(&mut self, dt: Duration) {
        self.delta = dt;
    }

    pub fn set_fixed_update_delta(&mut self, dt: Duration) {
        self.fixed_delta = dt;
    }

    pub fn delta(&self) -> Duration {
        if self._running_fixed_update {
            self.fixed_delta
        } else {
            self.delta
        }
    }
}
