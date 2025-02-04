//! Data Manipulation modules and functionalities

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use super::super::writer;
use super::super::reader::CsvMetadata;
use clap::Parser;

/// Command struct to hold the query, column name and output path

#[derive(Parser, Debug)]
pub struct Command {
    #[clap(long, value_delimiter = ',', help = "Query to filter the data comma separated")]
    pub query: Vec<String>,
    #[clap(long, value_delimiter = ',', help = "Column name to filter the data comma separated")]
    pub column: Vec<String>,
    #[clap(long, help = "Output path for the filtered data")]
    pub output_path: Option<String>,
    #[clap(subcommand)]
    pub subcommand: Subcommand
}

/// Subcommand enum to hold the different filtering functions

#[derive(Parser, Debug)]
pub enum Subcommand {
    #[clap(about = "Unsafe data filtering function")]
    Anyfilter(CsvMetadata),
    #[clap(about = "Safe data filtering function, single column & query matching")]
    Filter(CsvMetadata)
}

/// An iterator variant that yields strings or IO errors.
/// 
/// # Type Parameters
/// 
/// * `Item` - Each iteration produces a `Result<String, std::io::Error>`
///
/// The iterator is boxed to allow for dynamic dispatch.
///
pub enum FileRead {
    Iterator(Box<dyn Iterator<Item = Result<String, std::io::Error>>>),
    Reader(BufReader<File>)
}

/// impl block for the FileRead enum
impl FileRead {
    /// Returns an iterator over the lines of text within the file.
    ///
    /// # Returns
    ///
    /// Returns a boxed iterator that yields `Result<String, io::Error>` for each line.
    /// The iterator handles both cases where `FileRead` is constructed from an existing iterator
    /// or a direct file reader.
    ///
    pub fn lines(self) -> Box<dyn Iterator<Item = Result<String, io::Error>>> {
        match self {
            FileRead::Iterator(iter) => iter,
            FileRead::Reader(reader) => Box::new(reader.lines())
        }
    }
}

/// Filtering module contains functions to filter data from a csv file
pub mod filtering {
    use super::*;
    /// Unsafe data filtering function
    ///
    /// This function will take in a buffer reader and return a vector of filtered lines
    /// as strings, where each line contains at least one field that matches the query.
    /// # Unsafe
    /// This function can potentially provide unexpected results if the query if there are multiple
    /// fields in a line that match the query. The first field that matches the query will be
    /// considered as a match.
    pub fn any_filter(buffer: FileRead, filter_command: &Command, csv_struct: &CsvMetadata) -> Result<String, std::io::Error> {
        let mut writer = Vec::new();
        let querys = &filter_command.query;
        for (index, line) in buffer.lines().enumerate() {
            if index == 0 && csv_struct.has_header {
                match line {
                    Ok(header) => {
                        writer.push(vec![header]);
                    }
                    Err(e) => return Err(e),
                }
            } else{
                match line {
                    Ok(content) => {
                        let mut match_all = true;
                        for q in querys {
                            // Split the line by commas and check if any field matches the query
                            if content.split(',').any(|field| field.trim() == q) {}
                            else{ match_all = false;}
                        }
                        if match_all {
                            writer.push(vec![content]);
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
        }
        Ok(String::from("SUCCESS"))
    }
    /// Safe data filtering function, single column & query matching
    ///
    /// This function will take in a buffer reader, query and column name and return a vector of filtered lines
    /// as strings, where each line contains at least one field that matches the query.
    /// # Limitations
    /// This function will have undesired results if the csv elements contain commas within double quotes
    /// For example, due to the limitation of the split function, the following line will be split into 4 fields
    /// 1,"a,b",2,3 -> [1, "a, b", 2, 3]
    /// 
    /// Multi-Column filtering supports AND operation, i.e. all the queries must match all the respective columns;queries
    /// # Panics
    /// This function will panic if the column name is not found in the csv file
    pub fn filter(buffer: FileRead, filter_command: &Command, csv_struct: &CsvMetadata) -> Result<String, std::io::Error> {
        use super::*;

        let mut writer: Vec<Vec<String>> = Vec::new();
        let columns: &Vec<String> = &filter_command.column;
        let queries: &Vec<String> = &filter_command.query;
        let output_path = &filter_command.output_path;

        println!("Columns: {:?}", columns);
        println!("Queries: {:?}", queries);

        let mut column_indices = Vec::new();

        for (index, line) in buffer.lines().enumerate() {
            if index == 0 && csv_struct.has_header {
                // find the column indices
                match line {
                    Ok(header) => {
                        let headers: Vec<&str> = header.split(',').collect();
                        for column in columns {
                            match headers.iter().position(|&field| field.trim() == *column) {
                                Some(col_index) => column_indices.push(col_index),
                                None => panic!("\x1b[0;31mRuntime Panic:\x1b[0m Column {} not found in the csv file", column),
                            }
                        }
                        writer.push(vec![header]);
                    }
                    Err(e) => return Err(e),
                }
            } else {
                match line {
                    Ok(record) => {
                        let fields: Vec<&str> = record.split(',').collect();
                        let mut match_all = true;
                        for (col_index, query) in column_indices.iter().zip(queries) {
                            if fields[*col_index].trim() != *query {
                                match_all = false;
                                break;
                            }
                        }
                        if match_all {
                            writer.push(vec![record]);
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
            match &output_path {
                Some(path) => {
                    let _ = writer::csv_writer(path.clone(), writer.clone());
                },
                None => {}
            }
        }
        Ok(String::from("SUCCESS"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    #[test]
    fn test_filtering() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR")); //crate root
        path.push("test/example/data.csv");
        let csv_handler = CsvMetadata {
            file: path.to_str().unwrap().to_string(),
            delimiter: ',',
            has_header: true,
            column_types: vec!["string".to_string()]
        };
        let filter_command = Command {
            query: vec!["1".to_string()],
            column: vec!["key".to_string()],
            output_path: None,
            subcommand: Subcommand::Filter(csv_handler.clone())
        };
        let file = std::fs::File::open(path).unwrap();
        let reader = FileRead::Reader(BufReader::new(file));
        let writer = filtering::filter(reader, &filter_command, &csv_handler);
        assert!(writer.is_ok());
    }
    
    #[test]
    fn test_glob_reader_filtering() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR")); //crate root
        path.push("test/*/*.csv");
        let csv_handler = CsvMetadata {
            file: path.to_str().unwrap().to_string(),
            delimiter: ',',
            has_header: true,
            column_types: vec!["string".to_string()]
        };
        let filter_command = Command {
            query: vec!["1".to_string()],
            column: vec!["key".to_string()],
            output_path: None,
            subcommand: Subcommand::Filter(csv_handler.clone())
        };
        let reader = crate::reader::glob_reader(&csv_handler);
        let writer = filtering::filter(reader, &filter_command, &csv_handler);
        assert!(writer.is_ok());
    }
}
