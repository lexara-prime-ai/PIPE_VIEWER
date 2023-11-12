// Use BufReader & BufWriter -> to provide buffer behavior
use std::io::Result;
// Can't use crate:: here because the binary & library are separate crates
use cli_pipeviewer::{args::Args, read, stats, write};
// Arc -> Atomic Reference Counter
// Mutex will be used to protect access to mutable data
use crossbeam::channel::{bounded, unbounded};
// For spawning threads...
use std::thread;

fn main() -> Result<()> {
    let args = Args::parse();

    let Args {
        infile,
        outfile,
        silent,
    } = args;

    // Create communication channels
    let (stats_tx, stats_rx) = unbounded();
    // Set limit for the bounded channel to '1024'
    // If the limit is exceeded, the 1025th attempt will block until something is taken off the other end
    let (write_tx, write_rx) = bounded(1024);

    // Spawning a thread returns thread_handles
    let read_handle = thread::spawn(move || read::read_loop(&infile, stats_tx, write_tx));
    let stats_handle = thread::spawn(move || stats::stats_loop(silent, stats_rx));
    let write_handle = thread::spawn(move || write::write_loop(&outfile, write_rx));

    // Join threads -> crash main thread if any threads crash
    // .join() -> returns thread::Result<io::Result<()>>
    let read_io_result = read_handle.join().unwrap();
    let stats_io_result = stats_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();

    // Return an error if any threads returned an error
    read_io_result?;
    stats_io_result?;
    write_io_result?;

    // Preview silent using the dbg! macro -> one step above println -> it takes any arbitrary expression
    // dbg!(infile, outfile, silent);

    Ok(())
}
