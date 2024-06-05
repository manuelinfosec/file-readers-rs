# File Read Experiments in Rust

This repository contains a collection of experiments demonstrating various file reading strategies including:

- **BufRead**: Utilizes Rust's `BufReader` for buffered reading.
- **Chunks**: Reads files in chunks for improved performance with large files.
- **Default**: Implements the default file reading method using `std::fs::File`.
- **Threads**: Demonstrates multi-threaded file reading for parallel processing.
