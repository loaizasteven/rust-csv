use std::collections::HashMap;

pub mod argparser {
    use std::process::exit;

    fn parse(args:Vec<String>) -> Vec<String> {
        if args.len() < 3 {
            if args.len()>1 && args[1] == "--help" {
                println!("Usage: {} <--file> </path/to/file/>", args[0]);
                println!("--file: Path to the file to be read");
                exit(0);
            }
            panic!("Runtime Panic: {} --file /path/to/file/ \n Help: {} <--help> for more details", args[0], args[0]);
        }
        return args
    }

    pub fn parser(args:Vec<String>) -> super::HashMap<String, String> {
        let args: Vec<String> = parse(args);
        let mut map = super::HashMap::new();
        for i in 1..args.len() {
            if i % 2 == 1 {
                map.insert(args[i].strip_prefix("--").clone().unwrap().to_string(), args[i + 1].clone());
            }
        }
        map
    }
}