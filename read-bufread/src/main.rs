use std::env::args;
use std::fs::File;

fn main() {
    let args: Vec<String> = args().collect();
    let path: &String = &args[1];

    let file = File::open(path).expect("Unable to open file.");
    let file_length: usize = file
        .metadata()
        .expect("Unable to query file details")
        .len()
        .try_into()
        .expect("Cannot convert file length");
}
