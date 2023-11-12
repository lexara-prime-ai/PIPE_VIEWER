use std::io::Result;
use std::sync::mpsc::{Receiver, Sender};

pub fn stats_loop(
    silent: bool,
    stats_rx: Receiver<Vec<u8>>,
    write_tx: Sender<Vec<u8>>,
) -> Result<()> {
    let mut total_bytes = 0;
    loop {
        // Receive bytes/buffer from read thread
        let buffer = stats_rx.recv().unwrap();
        // Count number of bytes in buffer
        let num_bytes = buffer.len();
        total_bytes += num_bytes;

        // Update console output
        if !silent {
            // Use carriage return '\r' to return the cursor to beginning of the line
            eprint!("\rTotal bytes: {}", total_bytes);
        }
        // Send vector to write loop
        if write_tx.send(buffer).is_err() {
            break;
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
