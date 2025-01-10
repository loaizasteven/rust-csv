//! Provide [CsvMetadata] struct and [csv_reader] function to read csv files
use std::io::BufReader;
use std::fs::File;
use clap::Parser;

/// A reader defines the struct containing metadata of the csv file
/// # Example
/// Build a CsvMetadata struct
/// ```
/// # use sdk::reader::CsvMetadata;
/// let csv_handler = CsvMetadata{
///    file: "/path/to/file.csv".to_string(),
///    delimiter: ',',
///    has_header: true,
///    header: vec!["a".to_string(), "b".to_string()],
///    column_types: vec!["int".to_string(), "string".to_string()],
/// };
/// ```

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
pub struct CsvMetadata {
    #[clap(long)]
    pub file: String,
    #[clap(long)]
    pub delimiter: char,
    #[clap(long, action)]
    pub has_header: bool,
    #[clap(long, value_delimiter= ',')]
    pub header: Vec<String>,
    #[clap(long, value_delimiter= ',')]
    pub column_types: Vec<String>,
}

/// Reads a csv file and returns a `BufReader<File`
pub fn csv_reader(csv_struct: &CsvMetadata) -> BufReader<File> {
    let f = File::open(&csv_struct.file).expect("Error opening file");
    BufReader::new(f)
}