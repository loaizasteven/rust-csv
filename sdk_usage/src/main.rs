use sdk;

/// Main function that calls the sdk loader function
/// Function parse the command line arguments and calls the loader function
/// The binary crate is used for local development and testing of the sdk library
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let results = sdk::loader(args);

    match results {
        Ok(_) => println!("\x1b[0;32mSuccess\x1b[0m"),
        Err(e) => println!("\x1b[0;31mError\x1b[0m {}", e),
    }
}
