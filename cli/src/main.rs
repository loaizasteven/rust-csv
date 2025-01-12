use clap::Parser;
use sdk::data::manipulation;
use sdk;

#[derive(Parser)]
#[clap(
    about = "CLI tool to read csv files and perform filtering operations\n\
    ··············································\n\
    :░█▀▀░█▀█░█▀▀░▀█▀░░░█▀▀░█▀▀░█░█░░░▀█▀░░░█░█▀█:\n\
    :░█▀▀░█▀█░▀▀█░░█░░░░█░░░▀▀█░▀▄▀░░░░█░░▄▀░░█░█:\n\
    :░▀░░░▀░▀░▀▀▀░░▀░░░░▀▀▀░▀▀▀░░▀░░░░▀▀▀░▀░░░▀▀▀:\n\
    ··············································"
)]
enum Cli {
  Transform(manipulation::Command),
  Placeholder
}

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
