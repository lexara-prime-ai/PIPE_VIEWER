use crossbeam::channel::Receiver;
use std::io::Result;

pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>) -> Result<()> {
    let mut total_bytes = 0;
    loop {
        // Receive bytes/buffer from read thread
        let num_bytes = stats_rx.recv().unwrap();
        total_bytes += num_bytes;

        // Update console output
        if !silent {
            // Use carriage return '\r' to return the cursor to beginning of the line
            eprint!("\rTotal bytes: {}", total_bytes);
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
