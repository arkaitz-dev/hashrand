# hashrand

A versatile CLI tool that generates cryptographically secure random strings with multiple alphabet options and safety features.

## Description

`hashrand` generates cryptographically secure random strings using various alphabets. By default, it uses the base58 alphabet (Bitcoin alphabet), which excludes similar-looking characters like 0, O, I, and l for better readability. The tool also supports other alphabet configurations and can ensure generated strings don't collide with existing filenames.

## Installation

```bash
cargo install --path .
```

## Usage

```bash
hashrand [OPTIONS] [LENGTH]
```

Where `LENGTH` is an optional number between 2 and 128 that specifies the desired length of the generated hash. If not provided, defaults to 21.

### Options

- `-r, --raw`: Output without newline character (useful for piping or scripting)
- `--no-look-alike`: Use an alphabet that excludes commonly confused characters (0, O, I, l, 1)
- `--full`: Use full alphanumeric alphabet (uppercase, lowercase, and numbers 0-9)
- `--full-with-symbols`: Use full alphabet including symbols (-_*^@#+!?$%)
- `-c, --check`: Ensure the generated hash doesn't match any existing file or directory name in the current directory tree
- `--mkdir`: Create a directory with the generated hash as name
- `--touch`: Create a file with the generated hash as name
- `--prefix <PREFIX>`: Add a prefix before the generated hash
- `--suffix <SUFFIX>`: Add a suffix after the generated hash
- `--path <PATH>`: Specify the path where to create the file or directory

Notes:
- The alphabet options (`--no-look-alike`, `--full`, and `--full-with-symbols`) are mutually exclusive
- `--mkdir` and `--touch` are mutually exclusive
- When using `--mkdir` or `--touch`, the `--check` flag is automatically enabled to prevent naming conflicts
- `--prefix`, `--suffix`, and `--path` options require either `--mkdir` or `--touch`

### Examples

Generate a hash with default length (21 characters) using base58:
```bash
hashrand
```

Generate a 16-character hash without newline:
```bash
hashrand -r 16
```

Generate a 32-character hash that doesn't match any existing filename:
```bash
hashrand -c 32
```

Generate a hash using the no-look-alike alphabet:
```bash
hashrand --no-look-alike 24
```

Generate a hash with full alphanumeric characters:
```bash
hashrand --full 20
```

Generate a hash including symbols:
```bash
hashrand --full-with-symbols 16
```

Create a directory with a random name:
```bash
hashrand --mkdir
```

Create a file with a random name and custom length:
```bash
hashrand --touch 32
```

Create a directory with prefix and suffix:
```bash
hashrand --mkdir --prefix "temp_" --suffix "_data"
```

Create a file in a specific path:
```bash
hashrand --touch --path /tmp --prefix "session_"
```

Create a directory with no-look-alike alphabet and custom path:
```bash
hashrand --mkdir --no-look-alike --path ./backups --suffix "_backup"
```

## Alphabet Options

1. **Base58 (default)**: Bitcoin alphabet excluding 0, O, I, l
   - Characters: `123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz`
   - 58 characters total

2. **No Look-Alike**: Excludes commonly confused characters
   - Excludes: 0, O, I, l, 1
   - Characters: `23456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz`
   - 57 characters total

3. **Full Alphanumeric**: All letters and numbers
   - Characters: `0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz`
   - 62 characters total

4. **Full with Symbols**: Alphanumeric plus special characters
   - Characters: All alphanumeric plus `-_*^@#+!?$%`
   - 73 characters total

## Features

- **Multiple alphabet options** for different use cases
- **Cryptographically secure** random generation using nanoid
- **Customizable hash length** (2-128 characters)
- **Raw output mode** for scripting and piping
- **Collision detection** to avoid matching existing filenames
- **Directory and file creation** with random names
- **Prefix and suffix support** for structured naming
- **Custom path support** for organizing generated items
- **Fast and lightweight** with minimal dependencies
- **Comprehensive test suite** ensuring reliability

## Use Cases

- Generating unique identifiers for files or database records
- Creating temporary file names that won't collide
- Generating secure tokens or passwords
- Creating random test data
- Generating URL-safe random strings
- Creating organized temporary directories with prefixes/suffixes
- Batch file/directory creation with guaranteed unique names
- Session file management with structured naming

## Dependencies

- [nanoid](https://crates.io/crates/nanoid) - For secure random string generation
- [clap](https://crates.io/crates/clap) - For command-line argument parsing
- [walkdir](https://crates.io/crates/walkdir) - For recursive directory traversal (used with --check flag)

## License

This project is open source and available under the MIT License.