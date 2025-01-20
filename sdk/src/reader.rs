//! Provide [CsvMetadata] struct and [csv_reader] function to read csv files
use std::fs::File;
use clap::Parser;
use glob::glob;
use std::io::{SeekFrom, Seek, BufRead, BufReader};
use crate::data::manipulation::FileRead;

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
    #[clap(long, action, default_value = "true")]
    pub has_header: bool,
    #[clap(long, value_delimiter= ',', default_value = "string")]
    pub column_types: Vec<String>,
}

impl CsvMetadata {
    /// Instanstiate `self` and return clone of the CsvMetadata struct
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
pub fn csv_reader(csv_struct: &CsvMetadata) -> FileRead{
    let f = File::open(&csv_struct.file).expect("Error opening file");
    FileRead::Reader(BufReader::new(f))
}

pub fn glob_reader(csv_struct: &CsvMetadata) -> FileRead{
    let mut first_file:bool = true;
    let mut readers: Vec<BufReader<File>> = Vec::new();
    let mut pos: u64 = 0;

    for entry in glob(&csv_struct.file).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                println!("{:?}", path.display());
                if first_file {
                    let file_base: File = File::open(&path).expect("Error opening file");
                    let reader = BufReader::new(file_base);
                    readers.push(reader);
                    
                    let file_temp: File = File::open(&path).expect("Error opening file");
                    let mut temp_reader = BufReader::new(file_temp);
                    let mut temp_buffer: String = String::new();
                    temp_reader.read_line(&mut temp_buffer).expect("Unable to read temp file to buffer");
                    pos = temp_buffer.len() as u64;
                    first_file = false;
                }
                else {
                    let mut file_cont: File = File::open(&path).expect("Error opening file");
                    let _ = file_cont.seek(SeekFrom::Start(pos));
                    let reader = BufReader::new(file_cont);
                    readers.push(reader);
                }
            
            }
            Err(e) => eprintln!("{:?}", e),
        }
    }
    // Combine the readers into a single iterator
    return FileRead::Iterator(Box::new(readers.into_iter().flat_map(|reader| reader.lines())))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_glob_reader_multiple_csv() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR")); //crate root
        path.push("test/*/*.csv");
        let csv_handler = CsvMetadata {
            file: path.to_str().unwrap().to_string(),
            delimiter: ',',
            has_header: true,
            column_types: vec!["string".to_string()]
        };

        let result = glob_reader(&csv_handler);
        match result {
            FileRead::Iterator(iter) => {
                assert!(iter.count() > 0);
            },
            _ => panic!("Expected FileRead::Iterator")
        }
    }
}
