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

## Downloads

Pre-built binaries are available on the [Releases page](https://github.com/CNCSMonster/gen-mdbook-summary/releases).

### Linux x86_64 Versions

Two versions are available for Linux x86_64:

| Version | Size | Description |
|---------|------|-------------|
| **Standard** | ~1.2MB | Original binary, fast startup |
| **gzexe Compressed** | ~530KB | Compressed with gzexe, ~55% smaller, slightly slower startup (+15ms) |

**Which one to choose?**
- Use **Standard** for performance-critical scenarios or frequent invocations
- Use **gzexe Compressed** for bandwidth-constrained environments or storage-limited systems

> **Note:** The gzexe version is only available for Linux x86_64. Other platforms use the standard version.

## Documentation

- **[HELP.md](./HELP.md)** - Complete usage guide
- **[examples/](./examples/)** - Usage examples
- **Command line**: `gms --help`

## License

MIT - See [LICENSE](./LICENSE)