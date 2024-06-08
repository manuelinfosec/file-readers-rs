use std::env::args;
use std::fs::File;
use std::io::Read;
use std::thread;

fn main() {
    // Collect command-line arguments into a vector of strings
    let args: Vec<String> = args().collect();

    // Ensure the path to the file is provided as the second argument
    // Extracts a reference to the path string.
    let path: &String = &args.get(1).expect("File path not specified.");

    // Attempt to open the file at the given path
    // If the file cannot be opened, program panics.
    let mut file: File = File::open(path).expect("Unable to open file.");

    // Retrieve the metadata for the open file to get the length in `usize` bytes
    // If the metadata retrieval fails, program will panic
    let file_length: usize = file
        .metadata()
        .expect("Unable to query file details")
        .len()
        .try_into()
        .expect("Cannot convert file length");

    // Size of each block to be read from the file. 8MB in this case
    const BLOCK_SIZE: usize = 16_777_216;
    const THREADS: usize = 10;

    let division: usize = ((file_length / THREADS) as f64).ceil() as usize;

    // Use scoped threads for simplicity
    thread::scope(|scope: &thread::Scope| {});
}
