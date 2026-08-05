use crate::components::velocity::Velocity;
use bevy_ecs::component::Component;

#[derive(Component, Default, Debug)]
pub struct Position(pub f32, pub f32, pub (f32, f32));

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self(x, y, (x, y))
    }

    #[inline]
    pub fn note_previous(&mut self) {
        self.2.0 = self.0;
        self.2.1 = self.1;
    }

    #[inline]
    pub fn is_changed(&self) -> bool {
        (self.2.0 - self.0).abs() > f32::EPSILON || (self.2.1 - self.1).abs() > f32::EPSILON
    }

    #[inline]
    pub fn revert_previous(&mut self) {
        self.0 = self.2.0;
        self.1 = self.2.1;
    }

    pub fn add_velocity(&mut self, v: &Velocity, dt: &f32) -> &Self {
        self.0 += v.0 * dt;
        self.1 += v.1 * dt;

        // if !self.is_changed() {
        //     self.revert_previous();
        // }

        self
    }
}
