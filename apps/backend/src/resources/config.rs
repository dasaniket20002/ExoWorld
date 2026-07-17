use bevy_ecs::resource::Resource;

#[derive(Resource)]
pub struct Config {
    sim_speed: f32,
    pub default_tps: u16,

    pub default_fixed_update_interval: f32,
    pub logging_interval: f32,

    pub max_fixed_updates_per_frame: u16,
    pub max_entities: u32,

    pub world_size: (u32, u32),
    pub world_range: ((f32, f32), (f32, f32)),
}

impl Default for Config {
    fn default() -> Self {
        let _default_tps = 20;
        let _world_size = (1_000_000, 1_000_000);

        let range_x = (-(_world_size.0 as f32) / 2.0, (_world_size.0 as f32) / 2.0);
        let range_y = (-(_world_size.1 as f32) / 2.0, (_world_size.1 as f32) / 2.0);

        Self {
            sim_speed: 1.0,
            default_tps: _default_tps,

            default_fixed_update_interval: 1.0 / (_default_tps as f32),
            logging_interval: 2.0,

            max_fixed_updates_per_frame: 5,
            max_entities: 10_000_000,

            world_size: _world_size,
            world_range: (range_x, range_y),
        }
    }
}

impl Config {
    pub fn update_sim_speed(&mut self, _sim_speed: f32) {
        if _sim_speed >= 0.0 {
            self.sim_speed = _sim_speed;
        } else {
            eprintln!("Simulation Speed cannot be negative");
        }
    }

    pub fn expected_tps(&self) -> f32 {
        self.default_tps as f32 * self.sim_speed
    }

    pub fn fixed_update_interval(&self) -> f32 {
        self.default_fixed_update_interval * self.sim_speed
    }
}
