use sdk;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    sdk::loader(args);
}
