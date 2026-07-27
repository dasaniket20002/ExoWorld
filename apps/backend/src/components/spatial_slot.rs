use bevy_ecs::component::Component;

#[derive(Component, Clone, Copy)]
pub struct SpatialSlot(pub u32);
