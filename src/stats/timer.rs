use std::time::{Duration, Instant};

pub struct Timer {
    pub last_instant: Instant,
    pub delta: Duration,
    pub period: Duration,
    pub countdown: Duration,
    pub ready: bool,
}

impl Timer {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            last_instant: now,
            delta: Duration::default(),          // Default value -> 0
            period: Duration::from_millis(1000), // Timing 1s
            countdown: Duration::default(),      // Default value -> 0
            ready: true,                         // Set to true to get immediate progress output
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_instant;
        self.last_instant = now;
        // Decrement countdown
        // Durations can't be  negative -> need to use checked_sub to return a Result
        self.countdown = self.countdown.checked_sub(self.delta).unwrap_or_else(|| {
            self.ready = true;
            self.period
        });
    }
}

