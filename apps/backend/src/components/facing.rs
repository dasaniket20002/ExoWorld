use crate::utils::etc::fast_inv_sqrt;
use bevy_ecs::component::Component;

#[derive(Component, Default)]
pub struct Facing(f32, f32);

impl Facing {
    pub fn new(x: f32, y: f32) -> Self {
        let mag_sq = x * x + y * y;

        if mag_sq == 0.0 {
            return Self(0.0, 0.0); // Cannot normalize a zero vector
        }

        let inv_mag = fast_inv_sqrt(mag_sq);
        Self(x * inv_mag, y * inv_mag)
    }

    pub fn set(&mut self, x: f32, y: f32) -> &Self {
        let mag_sq = x * x + y * y;

        if mag_sq == 0.0 {
            self.0 = 0.0;
            self.1 = 0.0;

            return self;
        }

        let inv_mag = fast_inv_sqrt(mag_sq);
        self.0 = x * inv_mag;
        self.1 = y * inv_mag;

        self
    }

    pub fn get(&self) -> &Self {
        self
    }
}
