use std::fs::File;
// Use BufReader -> to provide buffer behavior
use crossbeam::channel::Receiver;
use std::io::{self, BufWriter, ErrorKind, Result, Write};

pub fn write_loop(outfile: &str, write_rx: Receiver<Vec<u8>>) -> Result<()> {
    // Create writer
    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    loop {
        // Receive vector|bytes from stats thread
        let buffer = write_rx.recv().unwrap();
        // Check if it's time to quit by checking for an empty buffer sentinel
        if buffer.is_empty() {
            // Exit cleanly
            break;
        }

        // Return Error Result
        if let Err(e) = writer.write_all(&buffer) {
            if e.kind() == ErrorKind::BrokenPipe {
                // stop program cleanly
                return Ok(());
            }
            return Err(e);
        }
    }
    // Keep going
    Ok(())
}
