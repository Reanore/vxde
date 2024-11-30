# vxde - VXD File Parser for Rust

`vxde` is a Rust library that provides a parser for `.vxd` files. These files contain key-value pairs used in games or configurations, where the values can be in plaintext or hashed. The parser handles files with flexible formatting, such as varying spaces, newlines, and semicolons as delimiters.

This crate allows you to easily parse `.vxd` files into a `HashMap` of variables, supporting a wide range of input formats.

## Features

- Parse `.vxd` files into a `HashMap` of key-value pairs.
- Handles whitespace, newlines, and semicolons as delimiters.
- Handles quoted values (e.g., `"value"`) and non-quoted values (e.g., `value`).
- Handles edge cases like empty files and invalid formats.
- Flexible input formatting for user convenience.

## Installation

To use `vxde` in your Rust project, do `cargo add vxde` or add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
vxde = "0.1.0"
```

## Usage

Here's a basic example of how to use the `VxdeParser` to read and parse a `.vxd` file:

### Example Code

```rust
use vxde::VxdeParser;

fn main() {
    // Read and parse a .vxd file
    let file_path = "path_to_your_file.vxd";
    match VxdeParser::from_file(file_path) {
        Ok(vxde) => {
            // Access the parsed variables
            let variables = vxde.get_variables();
            
            // Print each key-value pair
            for (key, value) in variables {
                println!("{} = {}", key, value);
            }
        }
        Err(e) => {
            eprintln!("Failed to parse file: {}", e);
        }
    }
}
```

### Example `.vxd` File Content

```plaintext
USER_ISLOGGEDIN=true;USER_NAME="reanore";USER_PASS="1234";
```

This will be parsed into a `HashMap` where the keys are `USER_ISLOGGEDIN`, `USER_NAME`, and `USER_PASS`, and the corresponding values will be `true`, `"reanore"`, and `"1234"`.

## API Documentation

### `VxdeParser`

The core struct of the library that handles parsing the `.vxd` file.

#### `VxdeParser::from_file(file_path: &str) -> Result<VxdeParser, io::Error>`

Reads a `.vxd` file from the given path and parses its contents.

##### Arguments:

- `file_path`: A string slice representing the path to the `.vxd` file.

##### Returns:

- `Ok(VxdeParser)` if the file was parsed successfully.
- `Err(io::Error)` if there was an error opening or reading the file.

#### `VxdeParser::get_variables(&self) -> &HashMap<String, String>`

Returns a reference to the `HashMap` containing the parsed variables.

##### Returns:

- A reference to a `HashMap<String, String>`, where the keys are the variable names and the values are the corresponding values from the `.vxd` file.

### Example Usage

```rust
use vxde::VxdeParser;

fn main() {
    let file_path = "config.vxd";
    let vxde = VxdeParser::from_file(file_path).unwrap();
    let variables = vxde.get_variables();

    println!("Parsed variables:");
    for (key, value) in variables {
        println!("{} = {}", key, value);
    }
}
```

## Tests

`vxde` includes a comprehensive test suite to ensure that the parser works correctly. These tests cover:

- Basic parsing of well-formed `.vxd` files.
- Handling of extra spaces, newlines, and semicolons as delimiters.
- Parsing empty files and files with missing values.
- Handling of malformed files and invalid formats.

You can run the tests using the following command:

```bash
cargo test
```

## Contributing

If you'd like to contribute to `vxde`, feel free to open an issue or submit a pull request. Contributions are always welcome!

### Steps for Contributing:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Write tests for your changes.
4. Run `cargo test` to ensure all tests pass.
5. Submit a pull request with a detailed explanation of your changes.

## License

`vxde` is licensed under the BSD 2-Clause License. See the [LICENSE](LICENSE) file for more details.

---

## Project Structure

```
vxde/
├── src/
│   ├── lib.rs  # Main library file containing the parser
│   └── tests/
│       └── mod.rs  # Tests for the VxdeParser
├── Cargo.toml  # The project's metadata and dependencies
└── README.md   # This README file
```

---

## Full Documentation of Core Methods

### `VxdeParser::from_file`

```rust
pub fn from_file(file_path: &str) -> io::Result<Self>
```

This function opens the given file and parses its contents. The file should follow the `.vxd` format, where each line can contain one or more key-value pairs, with keys and values separated by an equal sign (`=`) and pairs separated by semicolons (`;`).

- Whitespace, tabs, and newlines are ignored.
- Keys and values are parsed as strings. If a value is enclosed in double quotes (`"`), the quotes are removed from the value.
- Keys and values are trimmed of leading and trailing whitespace.

### `VxdeParser::get_variables`

```rust
pub fn get_variables(&self) -> &HashMap<String, String>
```

This function returns the `HashMap` that holds the parsed key-value pairs from the `.vxd` file. You can use this method to access the parsed variables.

## Example `.vxd` File Formats

The following are valid `.vxd` file formats:

### 1. Single Line, Clean Format

```plaintext
USER_ISLOGGEDIN=true;USER_NAME="reanore";USER_PASS="1234";
```

### 2. Spaced Out Format

```plaintext
USER_ISLOGGEDIN = true;
USER_NAME = "reanore";
USER_PASS = "1234";
```

### 3. Newlines Between Pairs

```plaintext
USER_ISLOGGEDIN=true;

USER_NAME="reanore";

USER_PASS="1234";
```

### 4. Mixed Spacing, Newlines, and Missing Values

```plaintext
USER_ISLOGGEDIN = true;

USER_NAME="reanore";

USER_PASS;
```

In the last example, the `USER_PASS` key has no value, and the parser will ignore it (i.e., no entry for `USER_PASS` will be in the `HashMap`).

---

That's everything! This README and documentation cover the usage, features, installation, and testing of the `vxde` crate with the BSD 2-Clause License included. Let me know if you'd like to add or modify anything!
