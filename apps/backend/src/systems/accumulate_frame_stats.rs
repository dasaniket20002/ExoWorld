use bevy_ecs::system::{Res, ResMut};

use crate::resources::{engine_stats::EngineStats, time::Time};

pub fn accumulate_frame_stats(time: Res<Time>, mut stats: ResMut<EngineStats>) {
    stats.delta_sum_current_window += time.delta();
    stats.frame_count_current_window += 1;
}
