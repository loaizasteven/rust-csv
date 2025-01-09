use std::io::Write;
use std::fs::File;

pub fn csv_writer(output_path: String, writer: Vec<Vec<String>>) -> Result<String, std::io::Error> {
    let file = File::create(output_path);
    match file {
        Ok(mut obj) => {
            for row in writer {
                let row_str: Vec<String> = row.iter().map(|s| s.to_string()).collect();
                let _ = obj.write_all(row_str.join(",").as_bytes());
                let _ = obj.write_all(b"\n");
                println!("DONE")
        }
        },
        Err(e) => return Err(e),
    }
    Ok(String::from("SUCCESS"))
}