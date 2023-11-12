use crate::CHUNK_SIZE;
use std::fs::File;
// Use BufReader -> to provide buffer behavior
use std::io::{self, BufReader, Read, Result};
use std::sync::mpsc::Sender;

// Since return values are no longer being used to communicate actual data
// the result will be an 'empty' result -> Result<Vec<u8>> TO Result<()>
pub fn read_loop(infile: &str, stats_tx: Sender<Vec<u8>>) -> Result<()> {
    // Create reader
    let mut reader: Box<dyn Read> = if !infile.is_empty() {
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    // Create buffer
    let mut buffer = [0; CHUNK_SIZE];

    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(bytes) => bytes,
            Err(_) => break,
        };
        // Send buffer to stats thread
        if stats_tx.send(Vec::from(&buffer[..num_read])).is_err() {
            break;
        };
    }
    // Send empty buffer to stats thread
    // Shut down if we run out of data to read
    let _ = stats_tx.send(Vec::new());
    Ok(())
}
