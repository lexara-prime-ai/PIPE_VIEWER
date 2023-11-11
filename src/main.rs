use clap::{App, Arg};
use std::env;
use std::fs::File;
// Use BufReader & BufWriter -> to provide buffer behavior
use std::io::{self, BufReader, BufWriter, ErrorKind, Read, Result, Write};

// Define CHUNK_SIZE -> constant for our buffer => 16 kilobytes
// const CHUNK_SIZE: usize = value in kilobytes;
const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> Result<()> {
    // Create placeholder for commandline input arguments
    let matches = App::new("pipe viewer")
        .arg(Arg::with_name("infile").help("Read from a file instead of stdin"))
        .arg(
            Arg::with_name("outfile")
                .short("o")
                .long("outfile")
                .takes_value(true)
                .help("Write output to a file instead of stdout"),
        )
        .arg(Arg::with_name("silent").short("s").long("silent"))
        .get_matches();
    let infile = matches.value_of("infile").unwrap_or_default();
    let outfile = matches.value_of("outfile").unwrap_or_default();
    // Create a new String in case there's an error -> unwrap_or(String...)
    // If the length is > 0, then environment variable is present and contains a value
    let silent = if matches.is_present("silent") {
        true
    } else {
        !env::var("PV_SILENT").unwrap_or_default().is_empty()
    };
    // Create reader
    let mut reader: Box<dyn Read> = if !infile.is_empty() {
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    // Create writer
    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    // Preview silent using the dbg! macro -> one step above println -> it takes any arbitrary expression
    // dbg!(infile, outfile, silent);
    let mut total_bytes = 0;
    // Create buffer
    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let num_read = match reader.read(&mut buffer) {
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
        if let Err(e) = writer.write_all(&buffer[..num_read]) {
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
