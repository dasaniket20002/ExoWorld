use crate::resources::time::UpdateTime;
use crate::resources::{config::Config, stats::Stats};
use crate::utils::schedules::Schedules;
use bevy_ecs::world::World;
use std::time::{Duration, Instant};

pub struct State {
    last_frame: Instant,

    fixed_accumulator: Duration,
    logging_accumulator: Duration,

    tps_accumulator: u16,
    tps_timer: Instant,
}

pub struct Runner {
    world: World,
    schedules: Schedules,
    state: State,
}

impl Runner {
    pub fn new(world: World, schedules: Schedules) -> Self {
        Self {
            world,
            schedules,
            state: State {
                last_frame: Instant::now(),
                fixed_accumulator: Duration::ZERO,
                logging_accumulator: Duration::ZERO,

                tps_accumulator: 0,
                tps_timer: Instant::now(),
            },
        }
    }

    pub fn run(&mut self) {
        loop {
            self.measure_frame();

            self.run_update_schedule();
            self.run_fixed_update_schedule();
            self.run_logging_schedule();

            self.update_stats();

            // Don't burn 100% CPU.
            // thread::yield_now();
        }
    }

    fn measure_frame(&mut self) {
        let frame_dt = self.state.last_frame.elapsed();
        self.state.last_frame = Instant::now();

        self.state.fixed_accumulator += frame_dt;
        self.state.logging_accumulator += frame_dt;

        let mut stats = self.world.resource_mut::<Stats>();
        stats.frame_time = frame_dt;

        let mut update_time = self.world.resource_mut::<UpdateTime>();
        update_time.delta = frame_dt;
    }

    fn run_update_schedule(&mut self) {
        self.schedules.update.run(&mut self.world);
    }

    fn run_fixed_update_schedule(&mut self) {
        let (fixed_update_interval, max_fixed_updates_per_frame) = {
            let config = self.world.resource::<Config>();
            (
                config.fixed_update_interval,
                config.max_fixed_updates_per_frame,
            )
        };

        let mut ticks_this_frame = 0;

        while self.state.fixed_accumulator >= fixed_update_interval
            && ticks_this_frame < max_fixed_updates_per_frame
        {
            ticks_this_frame += 1;
            self.schedules.fixed_update.run(&mut self.world);
            self.state.fixed_accumulator -= fixed_update_interval;
        }

        self.state.tps_accumulator += ticks_this_frame;
    }

    fn run_logging_schedule(&mut self) {
        let config = self.world.resource::<Config>();

        if self.state.logging_accumulator >= config.logging_interval {
            self.schedules.logging.run(&mut self.world);

            // Skip missed logging intervals.
            self.state.logging_accumulator = Duration::ZERO;
        }
    }

    fn update_stats(&mut self) {
        if self.state.tps_timer.elapsed() >= Duration::from_secs(1) {
            self.state.tps_timer = Instant::now();

            let mut stats = self.world.resource_mut::<Stats>();

            stats.current_tps = self.state.tps_accumulator;
            self.state.tps_accumulator = 0;

            // Running average TPS (EMA)
            if stats.average_tps == 0.0 {
                stats.average_tps = stats.current_tps as f32;
            } else {
                const ALPHA: f32 = 0.05;
                stats.average_tps =
                    stats.average_tps as f32 * (1.0 - ALPHA) + stats.current_tps as f32 * ALPHA;
            }

            // Min / Max TPS
            if stats.min_tps == 0 {
                stats.min_tps = stats.current_tps;
            } else {
                stats.min_tps = stats.min_tps.min(stats.current_tps);
            }

            stats.max_tps = stats.max_tps.max(stats.current_tps);
        }
    }
}
