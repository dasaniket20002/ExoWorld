use crate::resources::time::Time;
use crate::resources::{config::Config, stats::Stats};
use crate::utils::schedules::Schedules;
use bevy_ecs::world::World;
use std::thread;
use std::time::{Duration, Instant};

pub struct State {
    last_frame: Instant,

    fixed_update_accumulator: Duration,
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
                fixed_update_accumulator: Duration::ZERO,
                logging_accumulator: Duration::ZERO,

                tps_accumulator: 0,
                tps_timer: Instant::now(),
            },
        }
    }

    pub fn run(&mut self) {
        self.run_startup_schedule();

        loop {
            self.measure_frame();

            self.run_update_schedule();
            self.run_fixed_update_schedule();
            self.run_logging_schedule();

            self.update_stats();

            // Don't burn 100% CPU.
            thread::yield_now();
        }
    }

    fn run_startup_schedule(&mut self) {
        self.schedules.startup.run(&mut self.world);
    }

    fn measure_frame(&mut self) {
        let frame_dt = self.state.last_frame.elapsed();
        self.state.last_frame = Instant::now();

        self.state.fixed_update_accumulator += frame_dt;
        self.state.logging_accumulator += frame_dt;

        let mut stats = self.world.resource_mut::<Stats>();
        stats.frame_time = frame_dt;

        let mut time = self.world.resource_mut::<Time>();
        time.set_update_delta(frame_dt);
    }

    fn run_update_schedule(&mut self) {
        let mut time = self.world.resource_mut::<Time>();
        time.running_fixed_update(false);

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

        while self.state.fixed_update_accumulator >= fixed_update_interval
            && ticks_this_frame < max_fixed_updates_per_frame
        {
            let mut time = self.world.resource_mut::<Time>();
            time.running_fixed_update(true);
            time.set_fixed_update_delta(fixed_update_interval);

            self.schedules.fixed_update.run(&mut self.world);

            ticks_this_frame += 1;
            self.state.fixed_update_accumulator -= fixed_update_interval;
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
        let elapsed = self.state.tps_timer.elapsed();

        if elapsed >= Duration::from_secs(1) {
            self.state.tps_timer = Instant::now();

            let mut stats = self.world.resource_mut::<Stats>();

            let current_tps =
                ((self.state.tps_accumulator as f32) / elapsed.as_secs_f32()).round() as u16;
            stats.current_tps = current_tps;
            self.state.tps_accumulator = 0;

            // Running average TPS (EMA)
            if stats.average_tps == 0.0 {
                stats.average_tps = current_tps as f32;
            } else {
                const ALPHA: f32 = 0.05;
                stats.average_tps = stats.average_tps * (1.0 - ALPHA) + current_tps as f32 * ALPHA;
            }

            // Min / Max TPS
            if stats.min_tps == 0 {
                stats.min_tps = current_tps;
            } else {
                stats.min_tps = stats.min_tps.min(current_tps);
            }

            stats.max_tps = stats.max_tps.max(current_tps);
        }
    }
}
