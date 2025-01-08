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

use data::manipulation;

/// Loader function that reads a csv file and returns a BufReader
/// # Examples
/// See the sdk_usage crate for an example of how to use this function
pub fn loader(args: Vec<String>) -> std::io::BufReader<std::fs::File> {
    let arg_mapping = stdin_parser::argparser::parser(args);
    let csv_handler = reader::CsvMetadata{
        file: arg_mapping.get("file").unwrap().to_string(),
        delimiter: ',',
        has_header: true,
        header: vec!["a".to_string(), "b".to_string()],
        column_types: vec!["int".to_string(), "string".to_string()],
    };

    let reader = reader::csv_reader(&csv_handler);
    let writer = manipulation::filtering::filter(reader, "1", "key");
    println!("{:?}", writer);
    let reader = reader::csv_reader(&csv_handler);

    reader
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
