//! Data Manipulation modules and functionalities

use std::io::{BufReader, BufRead};
use std::fs::File;

pub mod filtering {
    /// Filtering data
    /// This function will take in a buffer reader and return a vector of filtered lines
    /// as strings, where each line contains at least one field that matches the query.
    use super::*;
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
