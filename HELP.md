# gen-mdbook-summary Help

## Quick Start

```bash
# Generate SUMMARY.md in src directory
gms -d src -o src/SUMMARY.md

# Or use default output location
gms
```

## Commands

### Main Command

```
Usage: gms [OPTIONS] [COMMAND]

Commands:
  init  initialize the .gmsignore file
  gen   generate the summary file
  help  Print this message or the help of the given subcommand(s)

Options:
  -d, --dir <DIR>        Optional name to operate on [default: src]
  -o, --output <OUTPUT>  specify the output file
  -s, --sort             if organize the items in order
  -i, --ignore <IGNORE>  specify the ignore file, using .gitignore grammar,
                         matched files will be ignored. [default: .gmsignore]
  -h, --help             Print help
  -V, --version          Print version
```

### `gms init`

Initialize the `.gmsignore` file with default ignore patterns.

```
Usage: gms init

Options:
  -h, --help  Print help
```

**Default ignore patterns:**
```
book.toml
.gitignore
book
*.doc
*.pdf
*.png
*.xlsx
*.pptx
*.jpg
```

### `gms gen`

Generate the `SUMMARY.md` file.

```
Usage: gms gen [OPTIONS]

Options:
  -d, --dir <DIR>        Optional name to operate on [default: src]
  -o, --output <OUTPUT>  specify the output file
  -s, --sort             if organize the items in order
  -i, --ignore <IGNORE>  specify the ignore file, using .gitignore grammar,
                         matched files will be ignored. [default: .gmsignore]
  -h, --help             Print help
```

## Usage Examples

### Basic Usage

```bash
# Generate SUMMARY.md in src directory
gms -d src -o src/SUMMARY.md

# Output to current directory
gms -d src -o SUMMARY.md

# Output to docs directory
gms -d src -o docs/SUMMARY.md
```

### With Custom Ignore File

```bash
# Use custom ignore file
gms -d src -o src/SUMMARY.md -i .myignore

# Use mdbook.ignore file
gms -d src -o src/SUMMARY.md -i mdbook.ignore
```

### Disable Sorting

```bash
# Generate without sorting
gms -d src -o src/SUMMARY.md --sort=false
```

## Output File Location

The tool supports outputting `SUMMARY.md` to any location:

| Output Location | Command |
|----------------|---------|
| `src/SUMMARY.md` | `gms -d src -o src/SUMMARY.md` |
| `./SUMMARY.md` | `gms -d src -o SUMMARY.md` |
| `docs/SUMMARY.md` | `gms -d src -o docs/SUMMARY.md` |

**Note:** The output file is automatically excluded from the generated summary.

## Ignore File

### Default Ignore File: `.gmsignore`

Create default ignore file:
```bash
gms init
```

### Ignore File Syntax

Uses `.gitignore` pattern syntax:

```
# Ignore specific files
*.pdf
*.png

# Ignore directories
book/

# Ignore by name
mdbook.ignore
```

### Common Patterns

```
# Book configuration
book.toml

# Build output
book/

# Images
*.png
*.jpg
*.gif

# Documents
*.pdf
*.doc
*.docx
*.xlsx
*.pptx
```

## Path Handling

### Relative Paths

Generated `SUMMARY.md` uses **relative paths** (relative to the `-d` directory):

```markdown
# Summary

- [Chapter 1](chapter1.md)
- [Chapter 2](chapter2.md)
```

### Special Characters

- **Spaces**: Automatically encoded as `%20`
- **Unicode**: Supported (Chinese, Japanese, etc.)

Example:
```
file with space.md  →  file%20with%20space.md
```

## Troubleshooting

### SUMMARY.md appears in output

The output file is automatically excluded. If you see it in the summary, make sure you specified the correct output path with `-o`.

### Files not ignored

Check your ignore file:
1. File exists at the specified location
2. Patterns use correct `.gitignore` syntax
3. File path is relative to the source directory

### mdbook fails to parse SUMMARY.md

Check for:
1. Correct Markdown link syntax: `- [Name](path.md)`
2. File paths are relative to the `SUMMARY.md` location
3. No special characters in paths (or properly encoded)
