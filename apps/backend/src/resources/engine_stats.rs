use bevy_ecs::resource::Resource;

#[derive(Resource, Default)]
pub struct EngineStats {
    pub ticks_last_window: u32,
    pub ticks_current_window: u32,
    pub measured_tps: f32,

    pub delta_sum_current_window: f32,
    pub frame_count_current_window: u32,
    pub avg_delta_last_window: f32,
    pub avg_fps_last_window: f32,
}
