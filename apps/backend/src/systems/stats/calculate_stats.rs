use bevy_ecs::system::{Res, ResMut};

use crate::resources::{config::Config, engine_stats::EngineStats};

pub fn calculate_stats(mut stats: ResMut<EngineStats>, cfg: Res<Config>) {
    stats.measured_tps = stats.ticks_current_window as f32 / cfg.logging_interval;
    stats.ticks_last_window = stats.ticks_current_window;

    if stats.frame_count_current_window > 0 {
        stats.avg_delta_last_window =
            stats.delta_sum_current_window / stats.frame_count_current_window as f32;
        stats.avg_fps_last_window = 1.0 / stats.avg_delta_last_window; // fps = 1 / delta
    }

    stats.ticks_current_window = 0;
    stats.delta_sum_current_window = 0.0;
    stats.frame_count_current_window = 0;
}
