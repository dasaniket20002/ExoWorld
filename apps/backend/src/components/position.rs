use crate::components::velocity::Velocity;
use bevy_ecs::component::Component;
use bevy_math::{Vec3A, vec3a};
use std::time::Duration;

#[derive(Component)]
pub struct Position(pub Vec3A);

impl Default for Position {
    fn default() -> Self {
        Self(Vec3A::ZERO)
    }
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(vec3a(x, y, z))
    }

    pub fn get(&self) -> Vec3A {
        self.0
    }

    pub fn add_velocity(&mut self, v: &Velocity, dt: &Duration) {
        self.0 += v.get() * dt.as_secs_f32();
    }
}
