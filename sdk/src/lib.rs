//! SDK for reading csv files efficiently from disk and providing command line argument parsing
//! for data manipulation. Current functionality provides filtering of large csv files reducing the I/O overhead from
//! high-level programming languages like Python
//! 
//! # Panics
//! If the file path is not provided as a command line argument, a panic will result

pub mod reader;
pub mod stdin_parser;
pub mod data;
pub mod writer;

use data::manipulation::FileRead;

/// Loader function that reads a csv file and returns a BufReader
pub fn loader(csv_handler: &reader::CsvMetadata) -> FileRead {
    if csv_handler.file.contains("*"){
        reader::glob_reader(csv_handler)
    }
    else {
        reader::csv_reader(csv_handler)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::Read, path::PathBuf};

    #[test]
    fn test_loader() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR")); //crate root
        path.push("test/example/data.csv");
        let csv_handler = reader::CsvMetadata {
            file: path.to_str().unwrap().to_string(),
            delimiter: ',',
            has_header: true,
            column_types: vec!["string".to_string()]
        };
        let result = super::loader(&csv_handler);
        let mut buffer =[0; 3];
        match result {
            FileRead::Reader(mut reader) => {
                let _ = reader.read(&mut buffer[..]);
                assert!(buffer == [b'k', b'e', b'y']);
            },
            _ => panic!("Expected FileRead::Reader")
        }
    }

}
