//! Data Manipulation modules and functionalities

use std::io::{BufReader, BufRead};
use std::fs::File;

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
        let writer = filtering::any_filter(reader, "1");
        assert_eq!(writer, vec!["1,'1'"]);
    }
}