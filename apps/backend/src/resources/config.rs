use bevy_ecs::resource::Resource;

pub const EXPECTED_TPS: u16 = 20;

#[derive(Resource)]
pub struct Config {
    pub sim_speed: i16,

    pub fixed_update_interval: f32,
    pub logging_interval: f32,

    pub max_fixed_updates_per_frame: u16,
    pub max_entities: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            sim_speed: 1,

            fixed_update_interval: 1.0 / (EXPECTED_TPS as f32),
            logging_interval: 2.0,

            max_fixed_updates_per_frame: 5,
            max_entities: 25000000,
        }
    }
}
