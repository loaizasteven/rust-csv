//! Provide [CsvMetadata] struct and [csv_reader] function to read csv files
use std::io::BufReader;
use std::fs::File;

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
pub struct CsvMetadata {
    pub file: String,
    pub delimiter: char,
    pub has_header: bool,
    pub header: Vec<String>,
    pub column_types: Vec<String>,
}

/// Reads a csv file and returns a `BufReader<File`
pub fn csv_reader(csv_struct: &CsvMetadata) -> BufReader<File> {
    let f = File::open(&csv_struct.file).expect("Error opening file");
    BufReader::new(f)
}