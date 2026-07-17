use crate::components::acceleration::Acceleration;
use bevy_ecs::component::Component;
use bevy_math::{Vec2, vec2};

#[derive(Component)]
pub struct Velocity(Vec2);

impl Default for Velocity {
    fn default() -> Self {
        Self(Vec2::ZERO)
    }
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self(vec2(x, y))
    }

    pub fn get(&self) -> Vec2 {
        self.0
    }

    pub fn add_acceleration(&mut self, a: &Acceleration, dt: &f32) {
        self.0 += a.get() * dt;
    }
}
