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
pub fn loader(csv_handler: reader::CsvMetadata, filter_command: manipulation::Command) -> Result<(), io::Error> {
    let reader = reader::csv_reader(&csv_handler);
    let _ = manipulation::filtering::filter(reader, &filter_command, &csv_handler);

    return Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

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
        let filter_command = data::manipulation::Command {
            query: vec!["1".to_string()],
            column: vec!["key".to_string()],
            output_path: None,
            subcommand: data::manipulation::Subcommand::Filter(csv_handler.clone())
        };
        let result = super::loader(csv_handler, filter_command);
        assert!(result.is_ok());
    }

}
