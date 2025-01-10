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

use std::io;

use data::manipulation;

/// Loader function that reads a csv file and returns a BufReader
/// # Examples
/// See the sdk_usage crate for an example of how to use this function
pub fn loader(csv_handler: reader::CsvMetadata) -> Result<(), io::Error> {
    let reader = reader::csv_reader(&csv_handler);
    let _ = manipulation::filtering::filter(reader, "1", "key", Some(String::from("test.csv")));

    return Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
