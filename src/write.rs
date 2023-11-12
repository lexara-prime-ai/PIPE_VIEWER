use std::fs::File;
// Use BufReader -> to provide buffer behavior
use std::io::{self, BufWriter, ErrorKind, Result, Write};

pub fn write(outfile: &str, buffer: &[u8]) -> Result<bool> {
    // Create writer
    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    // Return Error Result
    if let Err(e) = writer.write_all(buffer) {
        if e.kind() == ErrorKind::BrokenPipe {
            return Ok(false);
        }
        return Err(e);
    }
    // Keep going
    Ok(true)
}
