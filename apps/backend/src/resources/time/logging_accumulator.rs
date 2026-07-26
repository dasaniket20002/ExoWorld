use bevy_ecs::resource::Resource;

#[derive(Resource, Default)]
pub struct LoggingAccumulator {
    pub remainder: f32,
}
