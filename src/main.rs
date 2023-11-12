// Use BufReader & BufWriter -> to provide buffer behavior
use std::io::Result;
// Can't use crate:: here because the binary & library are separate crates
use pipeviewer::{args::Args, read, stats, write};

fn main() -> Result<()> {
    let args = Args::parse();
    // Preview silent using the dbg! macro -> one step above println -> it takes any arbitrary expression
    // dbg!(infile, outfile, silent);
    let mut total_bytes = 0;

    loop {
        let buffer = match read::read(&args.infile) {
            Ok(bytes) if bytes.is_empty() => break,
            Ok(bytes) => bytes,
            Err(_) => break,
        };

        stats::stats(args.silent, buffer.len(), &mut total_bytes, false);
        if !write::write(&args.outfile, &buffer)? {
            break;
        }
    }

    // Pass in '0' because we don't have a buffer
    stats::stats(args.silent, 0, &mut total_bytes, true);
    Ok(())
}
