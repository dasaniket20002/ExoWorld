use crate::components::velocity::Velocity;
use bevy_ecs::component::Component;
use bevy_math::{Vec2, vec2};

#[derive(Component)]
pub struct Position(Vec2);

impl Default for Position {
    fn default() -> Self {
        Self(Vec2::ZERO)
    }
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self(vec2(x, y))
    }

    pub fn get(&self) -> Vec2 {
        self.0
    }

    pub fn add_velocity(&mut self, v: &Velocity, dt: &f32) {
        self.0 += v.get() * dt;
    }
}
