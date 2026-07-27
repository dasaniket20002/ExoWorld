use bevy_ecs::resource::Resource;

#[derive(Resource, Default)]
pub struct CollisionPairs(pub Vec<(u32, u32)>);
