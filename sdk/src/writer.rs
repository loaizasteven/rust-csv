//! Provides [csv_writer] function to write a csv file to disk
use std::io::Write;
use std::fs::File;

/// Function to write a csv file to disk
/// 
/// # TODO:
/// When writing the file, the function includes a newline character at the EOF. 
pub fn csv_writer(output_path: String, writer: Vec<Vec<String>>) -> Result<String, std::io::Error> {
    let file = File::create(output_path);
    match file {
        Ok(mut obj) => {
            for row in writer {
                let row_str: Vec<String> = row.iter().map(|s| s.to_string()).collect();
                let _ = obj.write_all(row_str.join(",").as_bytes());
                let _ = obj.write_all(b"\n");
        }
        },
        Err(e) => return Err(e),
    }
    Ok(String::from("SUCCESS"))
}