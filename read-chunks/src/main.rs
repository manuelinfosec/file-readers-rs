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

    // Size of each block to be read from the file. 8MB in this case
    const BLOCK_SIZE: usize = 8_388_608;

    // Empty vector or u8 (bytes) with a length of 8MB, filled with zeros (0)
    // This buffer will hold the data read from the file
    let mut contents: Vec<u8> = vec![0_u8; BLOCK_SIZE];

    // Counter for total number of bytes read from the file
    let mut read_length: usize = 0;

    // Loop to read the file in chunks of 2MB.
    // The range is from 0 to the number of full blocks in the file can fit in the file
    // file_length / BLOCK_SIZE calculates how manu full blocks of 2MB
    for _ in 0..=(file_length / BLOCK_SIZE) {
        // Read a block of data from the file into the `contents` buffer
        // The read method also returns the number of bytes read, which is added to the counter
        // If reading fails at any point, there is panic
        read_length += file.read(&mut contents).expect("Couldn't read file.");
    }

    // Verify the entire file was read correctly by visualizing total number of bytes read from the file
    // and the original file length
    println!("Total bytes read: {read_length} bytes || Original file length: {file_length} bytes");
}
