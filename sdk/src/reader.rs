
use std::io::BufReader;
use std::fs::File;

pub struct CsvMetadata {
    pub file: String,
    pub delimiter: char,
    pub has_header: bool,
    pub header: Vec<String>,
    pub column_types: Vec<String>,
}

pub fn csv_reader(csv_struct: &CsvMetadata) -> BufReader<File> {
    let f = File::open(&csv_struct.file).expect("Error opening file");
    BufReader::new(f)
}