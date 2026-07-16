use bevy_ecs::world::World;
use std::time::Instant;

use crate::{
    resources::{
        config::Config,
        engine_stats::EngineStats,
        time::{FixedUpdateAccumulator, LoggingAccumulator, Time},
    },
    utils::schedules::{FixedUpdate, Logging, Startup, Update},
};

pub struct Runner {
    world: World,
}

impl Runner {
    pub fn new(world: World) -> Self {
        Self { world }
    }

    pub fn run(&mut self) {
        let mut last_frame = Instant::now();

        self.world.run_schedule(Startup);

        loop {
            let (fixed_update_interval, logging_interval, max_fixed_updates_per_frame) = {
                let cfg = self.world.resource::<Config>();
                (
                    cfg.fixed_update_interval(),
                    cfg.logging_interval,
                    cfg.max_fixed_updates_per_frame,
                )
            };

            let now = Instant::now();
            let delta = now.duration_since(last_frame).as_secs_f32();
            last_frame = now;

            {
                let mut time = self.world.resource_mut::<Time>();
                time.frame_start = now;
                time.set_update_delta(delta);
            }

            self.world.run_schedule(Update);

            {
                let mut acc = self.world.resource_mut::<FixedUpdateAccumulator>();
                acc.remainder += delta;
            }

            {
                let mut time = self.world.resource_mut::<Time>();
                time._running_fixed_update = true;
                time.set_fixed_update_delta(fixed_update_interval);
            }

            let mut ticks_this_frame = 0;
            while self.world.resource::<FixedUpdateAccumulator>().remainder >= fixed_update_interval
                && ticks_this_frame <= max_fixed_updates_per_frame
            {
                self.world
                    .resource_mut::<FixedUpdateAccumulator>()
                    .remainder -= fixed_update_interval;

                self.world.run_schedule(FixedUpdate);

                ticks_this_frame += 1;
                self.world
                    .resource_mut::<EngineStats>()
                    .ticks_current_window += 1;
            }

            {
                let mut time = self.world.resource_mut::<Time>();
                time._running_fixed_update = false;
            }

            {
                let mut log_acc = self.world.resource_mut::<LoggingAccumulator>();
                log_acc.remainder += delta;
            }

            if self.world.resource::<LoggingAccumulator>().remainder >= logging_interval {
                self.world.resource_mut::<LoggingAccumulator>().remainder -= logging_interval;

                {
                    let mut stats = self.world.resource_mut::<EngineStats>();
                    stats.measured_tps = stats.ticks_current_window as f32 / logging_interval;
                    stats.ticks_last_window = stats.ticks_current_window;

                    if stats.frame_count_current_window > 0 {
                        stats.avg_delta_last_window = stats.delta_sum_current_window
                            / stats.frame_count_current_window as f32;
                        stats.avg_fps_last_window = 1.0 / stats.avg_delta_last_window; // fps = 1 / delta
                    }

                    stats.ticks_current_window = 0;
                    stats.delta_sum_current_window = 0.0;
                    stats.frame_count_current_window = 0;
                }

                self.world.run_schedule(Logging);
            }

            // thread::yield_now();
        }
    }
}
