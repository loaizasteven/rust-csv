use sdk::reader::CsvMetadata;
use clap::Parser;

fn main() {
    let args = CsvMetadata::parse();
    let results = sdk::loader(args);

    match results {
        Ok(_) => println!("\x1b[0;32mSuccess\x1b[0m"),
        Err(e) => println!("\x1b[0;31mError\x1b[0m {}", e),
    }
}