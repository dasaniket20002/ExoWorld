use crate::components::acceleration::Acceleration;
use bevy_ecs::component::Component;
use bevy_math::{Vec3A, vec3a};

#[derive(Component)]
pub struct Velocity(Vec3A);

impl Default for Velocity {
    fn default() -> Self {
        Self(Vec3A::ZERO)
    }
}

impl Velocity {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(vec3a(x, y, z))
    }

    pub fn get(&self) -> Vec3A {
        self.0
    }

    pub fn add_acceleration(&mut self, a: &Acceleration, dt: &f32) {
        self.0 += a.get() * dt;
    }
}
