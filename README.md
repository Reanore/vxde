# vxde - VXD File Parser for Rust

`vxde` is a Rust library that provides a parser for `.vxd` files. These files contain key-value pairs used in games or configurations, where the values can be in plaintext or serialized. The parser handles files with flexible formatting, such as varying spaces, newlines, and semicolons as delimiters.

This crate allows you to easily parse `.vxd` files into a `HashMap` of variables, supporting a wide range of input formats.

## Series 2025 Features

- [x] Parse `.vxd` files into a `HashMap` of key-value pairs.
- [ ] Custom `vxde` serialization algorithm for encoding `.vxd` files.
- [ ] Custom `vxde` decoder for decoding serialized `.vxd` files back to plaintext.
- [x] Handle whitespace, newlines, and semicolons as delimiters.
- [x] Handle quoted values (e.g., `"value"`) and non-quoted values (e.g., `value`).
- [x] Handle edge cases like empty files and invalid formats.
- [ ] Support both plaintext and serialized `.vxd` files for parsing.
- [ ] Customizable serialization algorithm options.
- [ ] Support for nested key-value structures (like JSON).
- [ ] Improved error handling with detailed feedback for parsing.
- [ ] File validation to ensure `.vxd` structure is correct before parsing.
- [ ] Logging and debugging tools for better traceability.
- [ ] File format versioning support for future-proofing.
- [ ] Batch processing support for handling multiple `.vxd` files.
- [ ] Command-line tool for parsing, encoding, and decoding `.vxd` files.
- [ ] Performance optimizations for large `.vxd` files.
- [ ] Cross-platform support (Windows, macOS, Linux).
- [x] Integration tests to ensure all features work as expected.

### This version, Version 2.0.0 Series 2025, will have:
- A rework of the internal mechanisms of the parser
- A clearer syntax for the `.vxd` file, `NAME : TYPE = VALUE;`

## ReanoMeter Rating: C - Extensive changes 
| **Letter** | **Level**            |
|------------|----------------------|
| **S**      | 5 - Invisible Update |
| **A**      | 4 - Minor Adjustments|
| **B**      | 3 - Moderate Effort  |
| **C**      | 2 - Extensive Changes|
| **D**      | 1 - Complete Overhaul|

## Installation

To use `vxde` in your Rust project, run the command:

`cargo add vxde`

or add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
vxde = "2.0.0"
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
│   └── lib.rs  # Main library file containing the parser
├── tests/
│   ├── test_name.rs  # Tests for the VxdeParser
│   └── assets/
│       ├── test_data.vxd # assets for unit tests in the src files
│       ├── test_data2.vxd
│       └── test_name.rs/ # assets for the specific test files
│           └── func_name.vxd # assets for the specific function in named test file
├── Cargo.toml  # The project's metadata and dependencies
└── README.md   # This README file
```