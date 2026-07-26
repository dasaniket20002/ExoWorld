use crate::resources::{config::Config, engine_stats::EngineStats};
use bevy_ecs::system::Res;

pub fn log_stats_system(stats: Res<EngineStats>, cfg: Res<Config>) {
    println!(
        "[STATS] \
         tps: {:.2} / {:.2} | \
         avg_dt: {:.4} ms | \
         avg_ups: {:.1}",
        stats.measured_tps,
        cfg.expected_tps(),
        stats.avg_delta_last_window * 1000.0,
        stats.avg_fps_last_window,
    );
}
