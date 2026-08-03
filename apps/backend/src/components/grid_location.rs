use bevy_ecs::component::Component;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridLocation {
    pub chunk_id: usize,
    pub cell_id: usize,
}
