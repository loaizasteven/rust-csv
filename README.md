# crate::rust-csv
*Warning: This is an unstable version of the project. Use with caution as there may be breaking changes until the first stable release (1.0.0).*

## Overview
This project provides an SDK for reading CSV files efficiently from disk and includes functionalities for data manipulation. The SDK is designed to reduce I/O overhead from high-level programming languages like Python.

## Project Structure
- `sdk`: Contains the core library for reading and manipulating CSV files.
- `sdk_usage`: A binary crate used for local development and testing of the SDK library.

## Building the Project
To build the project, run the following command:
```sh
cargo build
```

## Generating Documentation
When pushing the code to the repo, use the following bash script as a wrapper for `cargo docs` to generate HTML, JS, and CSS for GitHub Pages.

```bash
bash cargo_docs_shortcut.sh
```

## TODO
- Add documentation to additional struct/enums
- Add descriptions for commands/sub-commands in cli tool
- Support for multi filtering
- Warning when filtering causes empty csv - No output / Force output command
- Add read me for use of cli (non-rust users)
- shell script for users to download cli tool for local development (via `curl`)
  - download latest binary release
  - Add logging with welcome msg and example usage
  - mv binary to /usr/.local/bin
- redesign of cli args, current initial command takes cli args and subcommand additional cli args. 
```bash
./cli transform \
        --query "'1'" \
        --column "val" \
        --output-path "./test.csv" \
      filter \
        --file "../sdk/test/example/data.csv"
```
- change println to logging instead.

## License
This project is licensed under the Apache-2.0 License. See the [LICENSE](LICENSE) file for details.
