use bevy_ecs::resource::Resource;

#[derive(Resource, Default)]
pub struct FixedUpdateAccumulator {
    pub remainder: f32,
}
