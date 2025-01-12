//! Provide [CsvMetadata] struct and [csv_reader] function to read csv files
use std::io::BufReader;
use std::fs::File;
use clap::Parser;

/// A reader module defines the struct containing metadata of the csv file
/// # Example
/// Build a CsvMetadata struct
/// ```
/// # use sdk::reader::CsvMetadata;
/// let metadata = CsvMetadata {
///     file: "/path/to/file.csv".to_string(),
///     delimiter: ',',
///     has_header: true,
///     column_types: vec!["string".to_string()]
/// };
/// ```

#[derive(Parser, Debug)]
#[clap(about = "CSV constructur")]
pub struct CsvMetadata {
    #[clap(long)]
    pub file: String,
    #[clap(long, default_value = ",")]
    pub delimiter: char,
    #[clap(long, action)]
    pub has_header: bool,
    #[clap(long, value_delimiter= ',', default_value = "string")]
    pub column_types: Vec<String>,
}

impl CsvMetadata {
    /// Returns the clone of the CsvMetadata struct
    pub fn clone(&self) -> CsvMetadata {
        CsvMetadata {
            file: self.file.clone(),
            delimiter: self.delimiter,
            has_header: self.has_header,
            column_types: self.column_types.clone()
        }
    }
}

/// Reads a csv file and returns a `BufReader<File`
pub fn csv_reader(csv_struct: &CsvMetadata) -> BufReader<File> {
    let f = File::open(&csv_struct.file).expect("Error opening file");
    BufReader::new(f)
}