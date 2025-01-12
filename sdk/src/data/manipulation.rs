//! Data Manipulation modules and functionalities

use std::fs::File;
use std::io::{BufRead, BufReader};
use super::super::writer;
use super::super::reader::CsvMetadata;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Command {
    #[clap(long)]
    pub query: String,
    #[clap(long)]
    pub column: String,
    #[clap(long)]
    pub output_path: Option<String>,
    #[clap(subcommand)]
    pub subcommand: Subcommand
}

#[derive(Parser, Debug)]
pub enum Subcommand {
    Anyfilter,
    Filter(CsvMetadata)
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
    pub fn any_filter(buffer: BufReader<File>, query: &str) -> Result<String, std::io::Error> {
        let mut writer = Vec::new();

        for line in buffer.lines() {
            match line {
                Ok(content) => {
                    // Split the line by commas and check if any field matches the query
                    if content.split(',').any(|field| field.trim() == query) {
                        writer.push(vec![content]); // Add the whole line to the result
                    }
                }
                Err(e) => return Err(e),
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
    /// # Panics
    /// This function will panic if the column name is not found in the csv file
    pub fn filter(buffer: BufReader<File>, filter_command: &Command, csv_struct: &CsvMetadata) -> Result<String, std::io::Error> {
        use super::*;

        let mut writer: Vec<Vec<String>> = Vec::new();
        let mut column_index = 0;
        let column = &filter_command.column;
        let query = &filter_command.query;
        let output_path = &filter_command.output_path;

        println!("Column: {}", column);
        println!("Query: {}", query);

        for (index, line) in buffer.lines().enumerate() {
            if index == 0 && csv_struct.has_header{
                // find the column index
                match line {
                    Ok(header) => {
                        let col_option = header.split(',').position(|field| field.trim() == column);
                        match col_option {
                            Some(col) => {
                                column_index = col;
                                writer.push(vec![header]); 
                            },
                            None => {
                                panic!("\x1b[0;31mRuntime Panic:\x1b[0m Column {} not found in the csv file", column);
                            }
                        }
                    }
                    Err(e) => return Err(e),
                }
            } else {
                match line {
                    Ok(content) => {
                        // Split the line by commas and check if any field matches the query

                        if content.split(',').nth(column_index).unwrap().trim() == query {
                            writer.push(vec![content]); // Add the whole line to the result
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
            query: "1".to_string(),
            column: "key".to_string(),
            output_path: None,
            subcommand: Subcommand::Filter(csv_handler.clone())
        };
        let file = std::fs::File::open(path).unwrap();
        let reader = BufReader::new(file);
        let writer = filtering::filter(reader, &filter_command, &csv_handler);
        assert!(writer.is_ok());
    }
}
