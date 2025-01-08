//! Data Manipulation modules and functionalities

use std::fs::File;
use std::io::{BufRead, BufReader};

/// Filtering module contains functions to filter data from a csv file
pub mod filtering {
    use super::*;
    // / Unsafe data filtering function
    // /
    // / This function will take in a buffer reader and return a vector of filtered lines
    // / as strings, where each line contains at least one field that matches the query.
    // / # Unsafe
    // / This function can potentially provide unexpected results if the query if there are multiple
    // / fields in a line that match the query. The first field that matches the query will be
    // / considered as a match.
    pub fn any_filter(buffer: BufReader<File>, query: &str) -> Vec<String> {
        let mut writer = Vec::new();

        for line in buffer.lines() {
            match line {
                Ok(content) => {
                    // Split the line by commas and check if any field matches the query
                    if content.split(',').any(|field| field.trim() == query) {
                        writer.push(content); // Add the whole line to the result
                    }
                }
                Err(e) => eprintln!("Error reading line: {}", e),
            }
        }
        writer
    }

    // / Safe data filtering function, single column & query matching
    // /
    // / This function will take in a buffer reader, query and column name and return a vector of filtered lines
    // / as strings, where each line contains at least one field that matches the query.
    // / # Limitations
    // / This function will have undesired results if the csv elements contain commas within double quotes
    // / For example, due to the limitation of the split function, the following line will be split into 4 fields
    // / 1,"a,b",2,3 -> [1, "a, b", 2, 3]
    // / # Panics
    // / This function will panic if the column name is not found in the csv file

    pub fn filter(buffer: BufReader<File>, query: &str, column: &str) -> Vec<String> {
        let mut writer = Vec::new();
        let mut column_index = 0;

        for (index, line) in buffer.lines().enumerate() {
            if index == 0 {
                // find the column index
                match line {
                    Ok(header) => {
                        let col_option = header.split(',').position(|field| field.trim() == column);
                        match col_option {
                            Some(col) => column_index = col,
                            None => {
                                panic!("\x1b[0;31mRuntime Panic:\x1b[0m Column {} not found in the csv file", column);
                            }
                        }
                    }
                    Err(e) => eprintln!("Error reading line: {}", e),
                }
            } else {
                match line {
                    Ok(content) => {
                        // Split the line by commas and check if any field matches the query

                        if content.split(',').nth(column_index).unwrap().trim() == query {
                            writer.push(content); // Add the whole line to the result
                        }
                    }
                    Err(e) => eprintln!("Error reading line: {}", e),
                }
            }
        }
        writer
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
        let file = std::fs::File::open(path).unwrap();
        let reader = BufReader::new(file);
        let writer = filtering::filter(reader, "'1'", "val");
        assert_eq!(writer, vec!["1,'1'"]);
    }
}