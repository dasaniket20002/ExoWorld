use std::collections::VecDeque;
use std::time::{Duration, Instant};

pub struct TpsTracker {
    /// Timestamp of each recent tick
    tick_times: VecDeque<Instant>,
    /// How far back to look
    window: Duration,
}

impl TpsTracker {
    pub fn new(window: Duration) -> Self {
        Self {
            tick_times: VecDeque::new(),
            window,
        }
    }

    /// Call this once per fixed update tick
    pub fn tick(&mut self) {
        let now = Instant::now();
        self.tick_times.push_back(now);
        self.evict(now);
    }

    /// Returns exact TPS over the sliding window
    pub fn tps(&mut self) -> f32 {
        let now = Instant::now();
        self.evict(now);

        self.tick_times.len() as f32 / self.window.as_secs_f32()
    }

    fn evict(&mut self, now: Instant) {
        let cutoff = now - self.window;
        while let Some(&front) = self.tick_times.front() {
            if front < cutoff {
                self.tick_times.pop_front();
            } else {
                break;
            }
        }
    }
}