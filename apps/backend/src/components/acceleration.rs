use bevy_ecs::component::Component;

#[derive(Component, Default)]
pub struct Acceleration(pub f32, pub f32);

impl Acceleration {
    pub fn new(x: f32, y: f32) -> Self {
        Self(x, y)
    }
}
