use std::env::args;
use std::fs::File;
use std::io::Read;

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

    // Empty vector or u8 (bytes) with size of file's length, filled with zeros (0)
    // This buffer will hold the data read from the file
    let mut contents: Vec<u8> = vec![0_u8; file_length];

    // Read all bytes until the end of the file (until EOF)
    let read_length: usize = file.read_to_end(&mut contents).expect("Couldn't read file");

    // Verify the entire file was read correctly by visualizing total number of bytes read from the file
    // and the original file length
    println!("Total bytes read: {read_length} bytes || Original file length: {file_length} bytes");
}
