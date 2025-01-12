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
///     column_types: vec!["string".to_string()],
///     query: "".to_string(),
///     column: "".to_string(),
///     output_path: None,
/// };
/// ```

#[derive(Parser, Debug)]
#[clap(about = "CLI tool to read csv files and perform filtering operations")]
pub struct CsvMetadata {
    #[clap(long)]
    pub file: String,
    #[clap(long, default_value = ",")]
    pub delimiter: char,
    #[clap(long, action)]
    pub has_header: bool,
    #[clap(long, value_delimiter= ',', default_value = "string")]
    pub column_types: Vec<String>,
    #[clap(long, default_value = "")]
    pub query: String,
    #[clap(long, default_value = "")]
    pub column: String,
    #[clap(long, default_value = None)]
    pub output_path: Option<String>,
}

/// Reads a csv file and returns a `BufReader<File`
pub fn csv_reader(csv_struct: &CsvMetadata) -> BufReader<File> {
    let f = File::open(&csv_struct.file).expect("Error opening file");
    BufReader::new(f)
}