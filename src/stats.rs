use crossbeam::channel::Receiver;
use std::io::Result;
use std::time::Instant;

pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>) -> Result<()> {
    let mut total_bytes = 0;
    let start = Instant::now();
    loop {
        // Receive bytes/buffer from read thread
        let num_bytes = stats_rx.recv().unwrap();
        total_bytes += num_bytes;

        // Update console output
        if !silent {
            // Use carriage return '\r' to return the cursor to beginning of the line
            eprint!("\rTotal bytes: {} Time taken: {}", total_bytes, start.elapsed().as_secs());
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
