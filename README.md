# File Read Experiments in Rust

This repository contains a collection of experiments demonstrating various file reading strategies including:

- **BufRead**: Utilizes Rust's `BufReader` for buffered reading.
- **Chunks**: Reads files in chunks for improved performance with large files.
- **Default**: Implements the default file reading method when "how to read a file in Rust" is Google'd.
- **IoRead**: Read raw IO bytes of whole file into a single buffer using `std::fs::File`.
- **Threads**: Demonstrates multi-threaded file reading for parallel processing.

## Results

Reading a 5GB file with the default method is very slow, more than 12 times slower than the best approach. Using IO read is a bit quicker since it doesn't require handling `String` allocations and conversions. Reading the file in blocks provides a noticeable speed improvement, and a buffered reader can manage this even more efficiently than doing it manually. Nonetheless, the biggest performance boost is seen with concurrent reads, which are almost 3 times faster than using a buffered reader. See the [Live Benchmark](https://github.com/manuelinfosec/file-readers-rs/actions) results.

![Benchmark Screenshot](/assets/image.png)
