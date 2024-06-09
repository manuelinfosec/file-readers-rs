use std::env::args;
use std::fs::{metadata, read_to_string};
fn main() {
    // Collect command-line arguments into a vector of strings
    let args: Vec<String> = args().collect();

    // Ensure the path to the file is provided as the second argument
    // Extracts a reference to the path string.
    let path: &String = &args.get(1).expect("File path not specified.");

    // Retrieve the metadata for the open file to get the length in `usize` bytes
    // If the metadata retrieval fails, program will panic
    let file_length: usize = metadata(&path)
        .expect("Unable to query file details")
        .len()
        .try_into()
        .expect("Cannot convert file length");

    // The simplest way of reading a file in Rust
    // https://doc.rust-lang.org/book/ch12-02-reading-a-file.html
    let contents: String = read_to_string(path).expect("Cannot read the file");

    // Verify the entire file was read correctly by visualizing total number of bytes read from the file
    // and the original file length
    println!("{} {file_length}", contents.len());
}
