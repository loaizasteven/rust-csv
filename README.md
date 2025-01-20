# crate::rust-csv
*Warning: This is an unstable version of the project. Use with caution as there may be breaking changes until the first stable release (1.0.0).*

## Overview
This project provides an SDK for reading CSV files efficiently from disk and includes functionalities for data manipulation. The SDK is designed to reduce I/O overhead from high-level programming languages like Python.

## Project Structure
- `sdk`: Contains the core library for reading and manipulating CSV files.
- `examples`: Contains example projects demonstrating how to use the SDK.
- `tests`: Includes integration tests for the SDK functionalities.
- `docs`: Documentation files and assets for the project.
- `cli`: Command-line interface for interacting with the SDK.

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
- Add documentation to additional structs/enums
- Support escaping entries with commas [val, "Another, val"]
- Support multi-filtering, OR operations
- Warning when filtering causes empty CSV - No output / Force output command
- Add README for CLI usage (non-Rust users)
- Shell script for users to download CLI tool for local development (via `curl`)
  - Download latest binary release
  - Add logging with welcome message and example usage
  - Move binary to /usr/local/bin
- Redesign CLI arguments; current initial command takes CLI args and subcommand additional CLI args.
```bash
./cli transform \
        --query 1,"'1'" \
        --column key,val \
        --output-path "./test.csv" \
      filter \
        --file "../sdk/test/example/data.csv"
```
- Change println to logging instead.

## License
This project is licensed under the Apache-2.0 License. See the [LICENSE](LICENSE) file for details.
