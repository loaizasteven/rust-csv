pub mod reader;
pub mod stdin_parser;

/// Loader function that reads a csv file and returns a BufReader
/// Function takes a vector of strings as input
/// # Examples
/// See the sdk_usage crate for an example of how to use this function
pub fn loader(args: Vec<String>) -> std::io::BufReader<std::fs::File> {
    let arg_mapping = stdin_parser::argparser::parser(args);
    let csv_handler = reader::CsvMetadata{
        file: arg_mapping.get("file").unwrap().to_string(),
        delimiter: ',',
        has_header: true,
        header: vec!["a".to_string(), "b".to_string()],
        column_types: vec!["int".to_string(), "string".to_string()],
    };
    
    return reader::csv_reader(&csv_handler)
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
