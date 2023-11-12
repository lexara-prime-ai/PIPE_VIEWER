use crate::CHUNK_SIZE;
use std::fs::File;
// Use BufReader -> to provide buffer behavior
use std::io::{self, BufReader, Read, Result};

pub fn read(infile: &str) -> Result<Vec<u8>> {
    // Create reader
    let mut reader: Box<dyn Read> = if !infile.is_empty() {
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    // Create buffer
    let mut buffer = [0; CHUNK_SIZE];

    let num_read = reader.read(&mut buffer)?;
    Ok(Vec::from(&buffer[..num_read]))
}
