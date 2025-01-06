pub mod reader;
pub mod stdin_parser;

/// Loader function that reads a csv file and returns a BufReader
/// Function takes a vector of strings as input
/// # Examples
/// See the sdk_usage crate for an example of how to use this function
pub fn loader(args: Vec<String>) {
    let my_file = stdin_parser::argparser::parser(args);
    let csv_handler = reader::CsvMetadata{
        file: my_file.get("file").unwrap().to_string(),
        delimiter: ',',
        has_header: true,
        header: vec!["a".to_string(), "b".to_string()],
        column_types: vec!["int".to_string(), "string".to_string()],
    };
    
    return reader::csv_reader(&csv_handler);
}

/// This is a simple function that adds two numbers together
/// # Examples
/// ```
/// let result = sdk::add(2, 2);
/// assert_eq!(result, 4);
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
