use bevy_ecs::component::Component;
use bevy_math::{Vec3, vec3};

#[derive(Component)]
pub struct Acceleration(Vec3);

impl Default for Acceleration {
    fn default() -> Self {
        Self(Vec3::ZERO)
    }
}

impl Acceleration {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(vec3(x, y, z))
    }

    pub fn get(&self) -> Vec3 {
        self.0
    }
}
