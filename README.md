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
## License
This project is licensed under the Apache-2.0 License. See the [LICENSE](LICENSE) file for details.
