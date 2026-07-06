use crate::resources::{config::Config, stats::Stats};
use bevy_ecs::system::Res;

pub fn log_stats(stats: Res<Stats>, config: Res<Config>) {
    if stats.current_tps < config.expected_tps {
        println!(
            "[WARN] Performance degraded: {} TPS (expected {})",
            stats.current_tps, config.expected_tps
        );
    }

    println!(
        "[STATS] Frame: {:.2}ms | TPS: {} (avg: {:.2}, min: {}, max: {}, budget: {}ms)",
        stats.frame_time.as_secs_f32() * 1000.0,
        stats.current_tps,
        stats.average_tps,
        stats.min_tps,
        stats.max_tps,
        config.fixed_update_interval.as_millis(),
    );
}
