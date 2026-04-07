# gen-mdbook-summary

A simple CLI tool to generate `SUMMARY.md` for [mdBook](https://github.com/rust-lang/mdBook) projects.

## Installation

```bash
cargo install gen-mdbook-summary
```

## Quick Start

```bash
# Initialize ignore file (optional)
gms init

# Generate SUMMARY.md
gms -d src -o src/SUMMARY.md

# Serve with mdbook
mdbook serve
```

## Documentation

- **[HELP.md](./HELP.md)** - Complete usage guide
- **[examples/](./examples/)** - Usage examples
- **Command line**: `gms --help`

## License

MIT - See [LICENSE](./LICENSE)