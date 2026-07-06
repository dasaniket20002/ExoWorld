use bevy_ecs::resource::Resource;
use std::time::Duration;

#[derive(Resource)]
pub struct UpdateTime {
    pub delta: Duration,
}

impl Default for UpdateTime {
    fn default() -> Self {
        Self {
            delta: Duration::ZERO,
        }
    }
}
