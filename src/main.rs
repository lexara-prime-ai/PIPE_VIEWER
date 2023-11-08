use std::env;
use std::io::{self, ErrorKind, Read, Result, Write};

// Define CHUNK_SIZE -> constant for our buffer => 16 kilobytes
// const CHUNK_SIZE: usize = value in kilobytes;
const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> Result<()> {
    // Create a new String in case there's an error -> unwrap_or(String...)
    // If the length is > 0, then environment variable is present and contains a value
    let silent = !env::var("PV_SILENT").unwrap_or_default().is_empty();
    // Preview silent using the dbg! macro -> one step above println -> it takes any arbitrary expression
    // dbg!(silent);
    let mut total_bytes = 0;
    // Create buffer
    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(bytes) => bytes,
            Err(_) => break,
        };
        total_bytes += num_read;
        // Update console output
        if !silent {
            // Use carriage return '\r' to return the cursor to beginning of the line
            eprint!("\rTotal bytes: {}", total_bytes);
        }
        // Return Error Result
        if let Err(e) = io::stdout().write_all(&buffer[..num_read]) {
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            return Err(e);
        }
    }
    if !silent {
        // Use carriage return '\r' to return the cursor to beginning of the line
        eprintln!("\rTotal bytes: {}", total_bytes);
    }

    Ok(())
}
