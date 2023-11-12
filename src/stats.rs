use crossbeam::channel::Receiver;
use std::io::Result;
use std::time::{Duration, Instant};

pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>) -> Result<()> {
    let mut total_bytes = 0;
    let start = Instant::now();
    let mut timer = Timer::new();
    loop {
        // Receive bytes/buffer from read thread
        let num_bytes = stats_rx.recv().unwrap();
        timer.update();
        // Calculate rate per second
        let rate_per_second = num_bytes as f64 / timer.delta.as_secs_f64();

        total_bytes += num_bytes;

        // Update console output
        if !silent && timer.ready {
            timer.ready = false;
            // Use carriage return '\r' to return the cursor to beginning of the line
            eprint!(
                "\rTotal bytes: {} Time taken: {} Bytes per sec: [{:.0}b/s]",
                total_bytes,
                // Implement custom trait
                start.elapsed().as_secs().as_time(),
                rate_per_second
            );
        }
        // Check if there's a need to quit, if so break out of the loop
        if num_bytes == 0 {
            break;
        }
    }
    if !silent {
        eprintln!();
    }
    Ok(())
}

// Custom trait implementation for u64
trait TimeOutput {
    fn as_time(&self) -> String;
}

impl TimeOutput for u64 {
    fn as_time(&self) -> String {
        // Divide by 3600 and mod i.e get Remainder by 3600
        // to get the amount of Hours and Minutes
        let (hours, left) = (*self / 3600, *self % 3600);
        // Perform calculation for minutes
        let (minutes, seconds) = (left / 60, left % 60);
        // Format hours, minutes & seconds
        // Make sure minutes & seconds are 0 padded
        format!("{}:{:02}:{:02}", hours, minutes, seconds)
    }
}

struct Timer {
    last_instant: Instant,
    delta: Duration,
    period: Duration,
    countdown: Duration,
    ready: bool,
}

impl Timer {
    fn new() -> Self {
        let now = Instant::now();
        Self {
            last_instant: now,
            delta: Duration::default(),          // Default value -> 0
            period: Duration::from_millis(1000), // Timing 1s
            countdown: Duration::default(),      // Default value -> 0
            ready: true,                         // Set to true to get immediate progress output
        }
    }

    fn update(&mut self) {
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
