use std::env;
use std::io::{self, Read, Write};

// Define CHUNK_SIZE -> constant for our buffer => 16 kilobytes
// const CHUNK_SIZE: usize = value in kilobytes;
const CHUNK_SIZE: usize = 16 * 1024;

fn main() {
    // Create a new String in case there's an error -> unwrap_or(String...)
    // If the length is > 0, then environment variable is present and contains a value
    let silent = !env::var("PV_SILENT").unwrap_or_default().is_empty();
    // Preview silent using the dbg! macro -> one step above println -> it takes any arbitrary expression
    // dbg!(silent);
    let mut total_bytes = 0;
    loop {
        let mut buffer = [0; CHUNK_SIZE];
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(bytes) => bytes,
            Err(_) => break,
        };
        total_bytes += num_read;
        io::stdout().write_all(&buffer[..num_read]).unwrap();
    }
    if !silent {
        eprintln!("Total bytes: {}", total_bytes);
    }
}
