pub mod args;
pub mod read;
pub mod stats;
pub mod write;

// Define CHUNK_SIZE -> constant for our buffer => 16 kilobytes
// const CHUNK_SIZE: usize = value in kilobytes;
const CHUNK_SIZE: usize = 16 * 1024;
