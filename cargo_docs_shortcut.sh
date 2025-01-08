# Shortcut for `cargo doc` with GitHub Pages support
cargo doc --no-deps
echo '<meta http-equiv="refresh" content="0; url=sdk/index.html">' > doc/index.html
rsync -av --remove-source-files doc/ docs/
rm -rf doc