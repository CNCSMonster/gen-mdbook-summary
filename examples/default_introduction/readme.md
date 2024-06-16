# Using README.md or readme.md as the default introduction file of the directory

```zsh
# cd current directory
cargo run -- -d src -o src/SUMMARY.md
# then you can see the SUMMARY.md in the src directory
cat src/SUMMARY.md
# then serve the book to see the result effect in the browser
mdbook serve
```
