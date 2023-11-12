mod timer;

use crossbeam::channel::Receiver;
use crossterm::{
    cursor, execute,
    style::{self, Color, PrintStyledContent},
    terminal::{Clear, ClearType},
};
use std::io::{self, Result, Stderr, Write};
use std::time::Instant;
use timer::Timer;

pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>) -> Result<()> {
    let mut total_bytes = 0;
    let start = Instant::now();
    let mut timer = Timer::new();
    let mut stderr = io::stderr();
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
            output_progress(
                &mut stderr,
                total_bytes,
                // Implement custom trait
                start.elapsed().as_secs().as_time(),
                rate_per_second,
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

fn output_progress(stderr: &mut Stderr, bytes: usize, elapsed: String, rate: f64) {
    // Convert parameter to String
    let bytes = style::style(format!("{} ", bytes)).with(Color::Red);
    let elapsed = style::style(elapsed).with(Color::Green);
    let rate = style::style(format!(" [{:.0}b/s]", rate)).with(Color::Blue);
    // Cue all 'cross term' commands and execute them all at once
    #[allow(deprecated)]
    let _ = execute!(
        stderr,
        cursor::MoveToColumn(0),
        Clear(ClearType::CurrentLine),
        PrintStyledContent(bytes),
        PrintStyledContent(elapsed),
        PrintStyledContent(rate)
    );
    let _ = stderr.flush();
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

// Create inline sub-module -> test
#[cfg(test)]
mod tests {
    // Since this is a sub module, use from super::TimeOutput trait
    use super::TimeOutput;

    #[test]
    fn as_time_format() {
        // This test checks if the TIME formatting is correct
        let pairs = vec![
            (5_u64, "0:00:05"),    // 5s -> 0h 00m 05s
            (60_u64, "0:01:00"),   // 60s -> 0h 01m 00s
            (154_u64, "0:02:34"),  // 120s + 34s -> 0h 02m 34s
            (3603_u64, "1:00:03"), // 3600s + 3s -> 1h 00m 3s
        ];
        for (input, output) in pairs {
            assert_eq!(input.as_time().as_str(), output);
        }
    }
}
