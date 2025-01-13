//! The CLI binary file is intended to be used by non-Rustaceans developers to interact with the rust-csv library.
//! Specifically written for Data Scientist and Data Engineers using high-level programming languages such as Python or R.
//! The CLI tool will allow users to read csv files and perform filtering operations, such as filtering by a specific column and query.
//! This simple tool will reduct the I/O overhead of reading csv files and filtering data, reducing the time spent on data preprocessing.
//! 
//! # Example
//! ```bash
//! cli transform \
//!  --query "'1'" \
//!  --column "val" \
//!  --output-path "./test.csv" \
//!  filter \
//!  --file "../sdk/test/example/data.csv"
//! ```
//! # Errors
//! This function will return an error if the command is not recognized or if there is an issue with the filtering operations.
//! 
use clap::Parser;
use sdk::data::manipulation;
use sdk;


/// Cli enum to hold the different commands
/// 

#[derive(Parser)]
#[clap(
    about = "
    ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░\n\
    CLI tool to read csv files and perform filtering operations\n\
    Main Documentation: https://github.com/loaizasteven/rust-csv\n\
    ··············································\n\
    :░█▀▀░█▀█░█▀▀░▀█▀░░░█▀▀░█▀▀░█░█░░░▀█▀░░░█░█▀█:\n\
    :░█▀▀░█▀█░▀▀█░░█░░░░█░░░▀▀█░▀▄▀░░░░█░░▄▀░░█░█:\n\
    :░▀░░░▀░▀░▀▀▀░░▀░░░░▀▀▀░▀▀▀░░▀░░░░▀▀▀░▀░░░▀▀▀:\n\
    ··············································\n\
    Developer: @stevenloiaza\n\
    ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░"
)]
enum Cli {
    /// CLI Transfrom entry point for CSV I/O operations, specifically filtering.
    /// See [Command]() for more information
    #[clap(about = "CSV I/O operations, specifically filtering")]
    Transform(manipulation::Command),
    /// Placeholder for additional cli commands
    #[clap(about = "Placeholder for additional cli commands")]
    Placeholder
}

/// Main entry point for the CLI tool to read csv files and perform filtering operations
/// # Arguments
/// * `args` - [Cli] enum to hold the different commands
/// # Panics
/// CLI entry point will panic if the command is not recognized. Additionally, it will 
/// inherit panics from the filtering functions

fn main() {
    let args = Cli::parse();
    let results: Result<String, std::io::Error> = match args {
        Cli::Transform(filter) => {
            let val = &filter.subcommand;
            let response = match val {
                manipulation::Subcommand::Filter(csv) => {
                    manipulation::filtering::filter(
                        sdk::reader::csv_reader(&csv),
                        &filter,
                        &csv
                    )
                },
                _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Unknown subcommand")),
            };

            response
        }
        _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Unknown command")),
    };

    match results {
        Ok(_) => {println!("\x1b[0;32mSuccess\x1b[0m")},
        Err(e) => {println!("\x1b[0;31mError\x1b[0m {}", e)},
    }
}
