use std::env::args;
use std::fs::{metadata, File};
use std::io::SeekFrom;
use std::io::{Read, Seek};
use std::thread;

fn main() {
    // Collect command-line arguments into a vector of strings
    let args: Vec<String> = args().collect();

    // Ensure the path to the file is provided as the second argument
    // Extracts a reference to the path string.
    let path: &String = &args.get(1).expect("File path not specified.");

    // // Retrieve the metadata for the open file to get the length in `usize` bytes
    // // If the metadata retrieval fails, program will panic
    let file_length: usize = metadata(&path)
        .expect("Unable to query file details")
        .len()
        .try_into()
        .expect("Cannot convert file length");

    // Size of each block to be read from the file (16MB)
    const BLOCK_SIZE: usize = 16_777_216;

    // Number of threads to be used for reading the file
    const THREADS: usize = 10;

    // Determine the portion of the file each thread will handle
    let division: usize = ((file_length / THREADS) as f64).ceil() as usize;

    // Use scoped threads to ensure all threads are joined before the main thread exits
    thread::scope(|scope: &thread::Scope| {
        // Create `THREADS` number of threads
        for i in 0..THREADS {
            scope.spawn(move || {
                // Open a file handle per thread
                // Ensuring each thread works on its own file descriptor
                let mut thread_file: File = File::open(&path).expect("Unable to open file");
                
                // Initialize a 16MB buffer to hold data read by this thread
                let mut contents: Vec<u8> = vec![0_u8; BLOCK_SIZE];

                // Counter for the number of bytes read in each operation
                // Init'd as 1, as zero would indicate an EOF
                let mut read_length: usize = 1;
                
                // Counter to keep track of the total nuber of bytes read by this thread
                let mut read_total: usize = 0;
                
                // Determing the offset in the file where this thread should start reading.
                // Each thred reads a different portion of the file
                let offset: u64 = (i * division) as u64;
                
                // Seek to the starting position in the file for this thread
                // If it couldn't start from there, the program panics
                thread_file
                    .seek(SeekFrom::Start(offset))
                    .expect("Couldn't seek to position in file");

                // Read data in iterations until the thread's portion is read or EOF is reached
                // i.e there is no more bytes to be read from the file
                while (read_total < division) && (read_length != 0) {
                    // Adjust the contents buffer size if the remaining bytes to be read are less than the block size
                    if read_total + BLOCK_SIZE > division {
                        // Reduce the vector to ensure we don't read beyond the division
                        // If this doesn't happen, data for the next thread will be read causing offset seek errors
                        contents.truncate(division - read_total);
                    }

                    // Read data into the contents buffer
                    // `read_length` is updated with the number of bytes read
                    read_length = thread_file.read(&mut contents).expect("Couldn't read file");
                    read_total += read_length;
                }

                // Verify the entire file was read correctly by visualizing total number of bytes read from the file
                // and the original file length
                println!("Total bytes read: {read_length} bytes || Original file length: {file_length} bytes");
            });
        }
    });
}
