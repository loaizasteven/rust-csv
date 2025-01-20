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

    /// Validate the csv file based on the file extension and headers for multiple files
    /// 
    /// The [CsvMetadata] struct is validated based on the file extension matching the `*.csv` extention.
    /// Additionally, for glob files the headers are validated to ensure that all files have the same headers. Otherwise, the concatenation
    /// of the files will result in a misaligned data structure. In other words, there will be shifts to the schema of the data.
    /// 
    /// # Note
    /// The [CsvMetadata::validate] function is a wrapper for two private functions **validate_extension** and **validate_multifile_header**
    /// If there are a large number of files, the validation process may take some time. This is caused by the need to read the headers of all files
    /// to check for consistency.
    /// 
    pub fn validate(&self)-> bool {
        if self.validate_extension() && self.validate_multifile_header() {
            return true;
        }
        return false;
    }

    fn validate_extension(&self) -> bool {
        let ext = self.file.split('.').last().unwrap();
        if ext == "csv" {
            return true;
        }
        return false;
    }

    fn validate_multifile_header(&self) -> bool {
        let mut header_list: Vec<String> = Vec::new();
        if self.has_header {
            for entry in glob(&self.file).expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => {
                        let file_base: File = File::open(&path).expect("Error opening file");
                        let mut reader = BufReader::new(file_base);
                        let mut buffer: String = String::new();
                        reader.read_line(&mut buffer).expect("Unable to read file to buffer");
                        header_list.push(buffer);
                    }
                    Err(e) => eprintln!("{:?}", e),
                }
            }

            let mut header_iter = header_list.iter();
            let first_header = header_iter.next().unwrap();
            for header in header_iter {
                if first_header != header {
                    return false
                }
            }
            return true
        }
        else {
            println!("No headers based on metadata");
            return true
        }
    }
}

/// Reads a csv file and returns a `BufReader<File`
pub fn csv_reader(csv_struct: &CsvMetadata) -> FileRead{
    let f = File::open(&csv_struct.file).expect("Error opening file");
    FileRead::Reader(BufReader::new(f))
}

/// Reads files matching the given glob pattern and returns an iterator over the contents of these files.
/// 
/// # Arguments
///
/// * `pattern` - A string slice that holds the glob pattern to match files.
///
/// # Returns
///
/// An iterator over the contents of the files that match the given glob pattern.
///
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

    fn constructer() -> CsvMetadata {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR")); //crate root
        path.push("test/*/*.csv");
        return CsvMetadata {
            file: path.to_str().unwrap().to_string(),
            delimiter: ',',
            has_header: true,
            column_types: vec!["string".to_string()]
        }
    }
    #[test]
    fn test_glob_reader_multiple_csv() {
        let csv_handler: CsvMetadata = constructer();

        let result = glob_reader(&csv_handler);
        match result {
            FileRead::Iterator(iter) => {
                assert!(iter.count() > 0);
            },
            _ => panic!("Expected FileRead::Iterator")
        }
    }

    #[test]
    fn test_validation() {
        let csv_handler: CsvMetadata = constructer();
        assert!(csv_handler.validate());
    }
}
