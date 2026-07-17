use bevy_ecs::component::Component;
use bevy_math::{Vec2, vec2};

#[derive(Component)]
pub struct Acceleration(Vec2);

impl Default for Acceleration {
    fn default() -> Self {
        Self(Vec2::ZERO)
    }
}

impl Acceleration {
    pub fn new(x: f32, y: f32) -> Self {
        Self(vec2(x, y))
    }

    pub fn get(&self) -> Vec2 {
        self.0
    }
}
