use bevy_ecs::component::Component;
use bevy_math::{Vec3A, vec3a};

#[derive(Component)]
pub struct Acceleration(Vec3A);

impl Default for Acceleration {
    fn default() -> Self {
        Self(Vec3A::ZERO)
    }
}

impl Acceleration {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(vec3a(x, y, z))
    }

    pub fn get(&self) -> Vec3A {
        self.0
    }
}
