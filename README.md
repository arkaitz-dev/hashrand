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

- `-r, --raw`: Output without newline character (useful for piping or scripting, works with all options)
- `--no-look-alike`: Use an alphabet that excludes commonly confused characters (0, O, I, l, 1)
- `--full`: Use full alphanumeric alphabet (uppercase, lowercase, and numbers 0-9)
- `--full-with-symbols`: Use full alphabet including symbols (-_*^@#+!?$%)
- `-c, --check`: Ensure the generated hash doesn't match any existing file or directory name in the current directory tree
- `--mkdir`: Create a directory with the generated hash as name
- `--touch`: Create a file with the generated hash as name
- `--prefix <PREFIX>`: Add a prefix before the generated hash
- `--suffix <SUFFIX>`: Add a suffix after the generated hash
- `--path <PATH>`: Specify the path where to create the file or directory
- `--api-key`: Generate a secure API key using full alphanumeric alphabet (format: ak_xxxxxxxx, 47 characters total, no customization allowed)
- `--password`: Generate a secure password using full alphabet with symbols (21 characters by default, length can be customized between 21-44)
- `--file-mode <MODE>`: Set file permissions when creating files (Unix-style octal, e.g., 644, 600)
- `--dir-mode <MODE>`: Set directory permissions when creating directories (Unix-style octal, e.g., 755, 700)
- `--audit-log`: Enable audit logging (outputs operations to stderr with timestamps)

Notes:
- The alphabet options (`--no-look-alike`, `--full`, `--full-with-symbols`, `--api-key`, and `--password`) are mutually exclusive
- `--mkdir` and `--touch` are mutually exclusive
- When using `--mkdir` or `--touch`, the `--check` flag is automatically enabled to prevent naming conflicts
- `--prefix`, `--suffix`, and `--path` options require either `--mkdir` or `--touch`
- `--api-key` cannot be combined with any other options (it generates a fixed 44-character key with ak_ prefix)
- `--password` can only be combined with a custom length parameter (21-44 characters)
- Permission options (`--file-mode`, `--dir-mode`) only work with `--touch` and `--mkdir` respectively
- `--audit-log` can be used with any operation and also controlled via `HASHRAND_AUDIT_LOG` environment variable

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

Generate a secure API key (format: ak_xxxxxxxx, 47 characters total):
```bash
hashrand --api-key
```

Generate a secure password with default length (21 characters):
```bash
hashrand --password
```

Generate a secure password with custom length:
```bash
hashrand --password 30
```

Create a file with specific permissions (Unix only):
```bash
hashrand --touch --file-mode 600
```

Create a directory with restricted permissions:
```bash
hashrand --mkdir --dir-mode 700 --prefix "secure_"
```

Generate with audit logging enabled:
```bash
hashrand --audit-log --mkdir
# Or via environment variable:
HASHRAND_AUDIT_LOG=1 hashrand --touch
```

Generate an API key without newline (for scripts):
```bash
hashrand --api-key --raw
# Output: ak_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

Generate a password without newline:
```bash
hashrand --password -r
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

5. **API Key Mode** (`--api-key`): Secure API key generation
   - Format: `ak_` + 44 random characters (47 total)
   - Uses full alphanumeric alphabet (62 characters)
   - Provides 256 bits of entropy for quantum-resistant security
   - Cannot be customized or combined with other options
   - Follows modern API key identification standards

6. **Password Mode** (`--password`): Secure password generation
   - Uses full alphabet with symbols (73 characters)
   - Default length: 21 characters (128 bits entropy)
   - Length can be customized (21-44 characters)
   - Cannot be combined with other options except length
   - Minimum 21 characters ensures cryptographic security

## Security Features

`hashrand` includes several security enhancements to ensure safe operation:

### Path Security
- **Path validation and canonicalization** prevents directory traversal attacks
- **Base path verification** ensures files/directories are created within intended locations
- **Permission validation** checks that target paths exist and are accessible

### Resource Protection
- **Directory traversal limits** (10 levels deep) prevent resource exhaustion
- **File count limits** (100,000 entries) protect against DoS during collision checking
- **Generation attempt limits** (1,000 tries) prevent infinite loops

### Audit and Compliance
- **Comprehensive audit logging** tracks all operations with timestamps
- **Environment variable support** (`HASHRAND_AUDIT_LOG`) for automated environments
- **No sensitive data logging** follows security best practices
- **Unix permissions control** allows setting specific file/directory permissions

### Error Handling
- **Graceful error handling** with informative messages instead of panics
- **Input validation** ensures all parameters are within safe ranges
- **Secure defaults** maintain security when optional parameters aren't specified

## Features

- **Multiple alphabet options** for different use cases
- **Cryptographically secure** random generation using nanoid
- **Customizable hash length** (2-128 characters)
- **Raw output mode** for scripting and piping
- **Collision detection** to avoid matching existing filenames
- **Directory and file creation** with random names
- **Prefix and suffix support** for structured naming
- **Custom path support** for organizing generated items
- **Unix file permissions control** for secure file/directory creation
- **Audit logging system** for tracking operations and compliance
- **Security hardening** with path validation and resource limits
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
- Generating secure API keys for authentication
- Creating strong passwords for user accounts or services
- **Security-focused scenarios:**
  - Creating secure temporary files with restricted permissions
  - Generating auditable random identifiers for compliance
  - Setting up secure directories for sensitive data processing
  - Creating trackable session directories with audit trails

## Dependencies

- [nanoid](https://crates.io/crates/nanoid) - For secure random string generation
- [clap](https://crates.io/crates/clap) - For command-line argument parsing
- [walkdir](https://crates.io/crates/walkdir) - For recursive directory traversal (used with --check flag)

## License

This project is open source and available under the MIT License.