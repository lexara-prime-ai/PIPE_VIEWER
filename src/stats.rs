pub fn stats(silent: bool, num_read: usize, total_bytes: &mut usize, last: bool) {
    *total_bytes += num_read;

    // Update console output
    if !silent {
        // Use carriage return '\r' to return the cursor to beginning of the line
        eprint!("\rTotal bytes: {}", total_bytes);
        if last {
            eprintln!();
        }
    }
}
