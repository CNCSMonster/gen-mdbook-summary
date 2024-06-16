# gen-mdbook-summary

## Description

This is a simple tool to generate a `SUMMARY.md` file for a [mdBook](https://github.com/rust-lang/mdBook) project.

## Installation and Usage


quick start:

```zsh
# install the tool
cargo install gen-mdbook-summary
# create a mdbook project
mkdir my-mdbook && cd my-mdbook && mdbook init
# create a ignore file for gen-mdbook-summary
echo "**/readme.md" > mdbook.ignore
# generate a summary file
gms -d src -o src/SUMMARY.md
```

for more usage, please check the help message:

```zsh
gms --help
```

## License

[LICENSE](./LICENSE)