use crate::components::velocity::Velocity;
use bevy_ecs::component::Component;
use bevy_math::{Vec3, vec3};

#[derive(Component)]
pub struct Position(pub Vec3);

impl Default for Position {
    fn default() -> Self {
        Self(Vec3::ZERO)
    }
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(vec3(x, y, z))
    }

    pub fn get(&self) -> Vec3 {
        self.0
    }

    pub fn add_velocity(&mut self, v: &Velocity, dt: &f32) {
        self.0 += v.get() * dt;
    }
}
