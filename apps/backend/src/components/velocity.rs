use crate::components::acceleration::Acceleration;
use bevy_ecs::component::Component;

#[derive(Component, Default)]
pub struct Velocity(pub f32, pub f32);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self(x, y)
    }

    pub fn add_acceleration(&mut self, a: &Acceleration, dt: &f32) -> &Self {
        self.0 += a.0 * dt;
        self.1 += a.1 * dt;

        self
    }
}
