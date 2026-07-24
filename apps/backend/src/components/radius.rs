use bevy_ecs::component::Component;

// this stores an u8, but it is supposed to be used like Radius / 100; so that 250 actually means 2.5; max = 255
#[derive(Component)]
pub struct Radius(pub u8);

impl Default for Radius {
    fn default() -> Self {
        Self(100)
    }
}
