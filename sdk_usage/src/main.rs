use sdk;

/// Main function that calls the sdk loader function
/// Function parse the command line arguments and calls the loader function
/// The binary crate is used for local development and testing of the sdk library
fn main() {
    let args: Vec<String> = std::env::args().collect();
    sdk::loader(args);
}
