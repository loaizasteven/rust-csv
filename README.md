# rust-csv
*Warning: This is an unstable version of the project. Use with caution as there may be breaking changes until the first stable release (1.0.0).*

### Note on `cargo doc`
When pushing the code to the repo follow the guidelines below to ensure the documentation is reflected in the github pages for this project.

```bash
cargo doc
echo '<meta http-equiv="refresh" content="0; url=sdk/index.html">' > doc/index.html
rsync -av --remove-source-files doc/ docs/
```