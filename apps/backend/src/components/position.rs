use crate::components::velocity::Velocity;
use bevy_ecs::component::Component;

#[derive(Component, Default)]
pub struct Position(pub f32, pub f32);

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self(x, y)
    }

    pub fn add_velocity(&mut self, v: &Velocity, dt: &f32) -> &Self {
        self.0 += v.0 * dt;
        self.1 += v.1 * dt;

        self
    }
}
