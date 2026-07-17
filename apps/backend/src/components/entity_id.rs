use bevy_ecs::component::Component;
use uuid::Uuid;

#[derive(Component)]
pub struct EntityID(Uuid);

impl Default for EntityID {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl EntityID {
    pub fn get(&self) -> Uuid {
        self.0
    }
}
