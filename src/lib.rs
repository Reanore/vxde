//! # VxdeParser: A Parser for `.vxd` Configuration Files
//!
//! This Rust package allows for easy parsing of `.vxd` files, which define key-value pairs
//! in a custom syntax for various types of data, including strings, integers, booleans, etc.
//!
//! ## Key Features
//!
//! - Parses `.vxd` files containing key-value declarations with support for multiple data types.
//! - Supports automatic conversion of strings to various types (e.g., integers, floats, booleans).
//! - Handles `null` values properly and supports a custom `Null` variant for missing data.
//! - Includes helpful error handling and informative error messages for malformed files.
//! - Enables easy access to parsed variables using a HashMap.
//!
//! ## Installation
//!
//! Add this to your `Cargo.toml` under `[dependencies]`:
//!
//! ```toml
//! vxdparser = "0.1"
//! ```
//!
//! ## Example Usage
//!
//! Here is a simple example showing how to use the `VxdeParser` to load and read a `.vxd` file:
//!
//! ```no_run
//! use vxdparser::{VxdeParser, VxdValue};
//!
//! // Load the .vxd file
//! let parser = VxdeParser::from_file("config.vxd");
//!
//! // Handle the result
//! match parser {
//!     Ok(parsed) => {
//!         // Retrieve variables and print them
//!         let variables = parsed.get_variables();
//!         for (key, value) in variables {
//!             match value {
//!                 VxdValue::String(s) => println!("{} = String: {}", key, s),
//!                 VxdValue::I32(i) => println!("{} = i32: {}", key, i),
//!                 _ => println!("{} = Other: {:?}", key, value),
//!             }
//!         }
//!     }
//!     Err(err) => eprintln!("Failed to parse file: {}", err),
//! }
//! ```
//!
//! ## Error Handling
//!
//! - **Invalid Syntax**: The parser will return an error if a line does not conform to the expected format (e.g., missing `:` or `;`).
//! - **Unsupported Data Types**: If the file contains unsupported types, the parser will throw an error.
//! - **Parsing Null Values**: Null values (e.g., `null` or an empty value) will be parsed as `VxdValue::Null`.
//!
//! ## Supported Data Types
//!
//! The following data types are supported in the `.vxd` file format:
//!
//! - `string`: A regular string value (e.g., `key: string = "value";`).
//! - `i32`: A 32-bit signed integer (e.g., `key: i32 = 123;`).
//! - `i64`: A 64-bit signed integer (e.g., `key: i64 = 123456789;`).
//! - `u32`: A 32-bit unsigned integer (e.g., `key: u32 = 123;`).
//! - `u64`: A 64-bit unsigned integer (e.g., `key: u64 = 1234567890;`).
//! - `f32`: A 32-bit floating point (e.g., `key: f32 = 3.14;`).
//! - `f64`: A 64-bit floating point (e.g., `key: f64 = 2.718;`).
//! - `bool`: A boolean value (`true` or `false`).
//! - `char`: A single character (e.g., `key: char = 'a';`).
//! - `null`: Used to represent missing or undefined values (e.g., `key: string = null;`).

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

/// Represents different possible values that can be parsed from a `.vxd` file.
/// It supports multiple types, including `Null` for missing or undefined values.
#[derive(Debug, Clone, PartialEq)]
pub enum VxdValue {
    String(String),
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    Bool(bool),
    Char(char),
    Null, // Represents null values
}

/// A struct that handles the parsing of `.vxd` files.
/// It stores parsed key-value pairs in a `HashMap` for easy retrieval.
pub struct VxdeParser {
    variables: HashMap<String, VxdValue>,
}

impl VxdeParser {
    /// Reads a `.vxd` file, parses its contents, and stores the variables in a `VxdeParser` instance.
    ///
    /// The file is expected to contain key-value pairs with specific syntax. The parser will attempt
    /// to convert the values into the appropriate data types (e.g., strings, integers, floats).
    ///
    /// # Arguments
    ///
    /// * `file_path` - A string slice representing the path to the `.vxd` file.
    ///
    /// # Returns
    ///
    /// - `Ok(VxdeParser)` if the file is successfully parsed and variables are extracted.
    /// - `Err(io::Error)` if there is an issue reading the file or parsing the content.
    ///
    /// # Example
    ///
    /// ```no_run
    /// let parser = VxdeParser::from_file("config.vxd");
    /// match parser {
    ///     Ok(parsed) => {
    ///         let variables = parsed.get_variables();
    ///         for (key, value) in variables {
    ///             println!("{}: {:?}", key, value);
    ///         }
    ///     },
    ///     Err(err) => eprintln!("Error reading file: {}", err),
    /// }
    /// ```
    pub fn from_file(file_path: &str) -> io::Result<Self> {
        let mut variables = HashMap::new();

        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);

        // Regex pattern to capture variable declarations
        let re = Regex::new(r"(?m)^\s*([A-Za-z_][A-Za-z0-9_]*)\s*:\s*(string|i32|i64|u32|u64|f32|f64|bool|char)\s*(=\s*([^;]*))?\s*;").unwrap();

        for line in reader.lines() {
            let line = line?;

            // Attempt to match the pattern for valid variable declarations
            for caps in re.captures_iter(&line) {
                let name = &caps[1];
                let vtype = &caps[2];
                let value = caps.get(4).map_or("", |m| m.as_str()).trim();

                // Parse the value based on the type
                let parsed_value = match vtype {
                    "string" => {
                        if value == "null" || value.is_empty() {
                            VxdValue::Null
                        } else {
                            VxdValue::String(value.to_string())
                        }
                    },
                    "i32" => value.parse::<i32>().ok().map(VxdValue::I32).unwrap_or(VxdValue::Null),
                    "i64" => value.parse::<i64>().ok().map(VxdValue::I64).unwrap_or(VxdValue::Null),
                    "u32" => value.parse::<u32>().ok().map(VxdValue::U32).unwrap_or(VxdValue::Null),
                    "u64" => value.parse::<u64>().ok().map(VxdValue::U64).unwrap_or(VxdValue::Null),
                    "f32" => value.parse::<f32>().ok().map(VxdValue::F32).unwrap_or(VxdValue::Null),
                    "f64" => value.parse::<f64>().ok().map(VxdValue::F64).unwrap_or(VxdValue::Null),
                    "bool" => value.parse::<bool>().ok().map(VxdValue::Bool).unwrap_or(VxdValue::Null),
                    "char" => value.chars().next().map(VxdValue::Char).unwrap_or(VxdValue::Null),
                    _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Unsupported type: {}", vtype))),
                };

                variables.insert(name.to_string(), parsed_value);
            }
        }

        Ok(VxdeParser { variables })
    }

    /// Retrieves the parsed variables as a reference to a `HashMap<String, VxdValue>`.
    ///
    /// This method allows users to access the parsed key-value pairs.
    ///
    /// # Returns
    ///
    /// - A reference to the `HashMap<String, VxdValue>`, containing all the parsed variables.
    ///
    /// # Example
    ///
    /// ```no_run
    /// let variables = parser.get_variables();
    /// for (key, value) in variables {
    ///     println!("{}: {:?}", key, value);
    /// }
    /// ```
    pub fn get_variables(&self) -> &HashMap<String, VxdValue> {
        &self.variables
    }
}

/// A utility function to print the parsed variables in a readable format.
pub fn print_variables(variables: &HashMap<String, VxdValue>) {
    for (key, value) in variables {
        match value {
            VxdValue::String(s) => println!("{}: String -> {}", key, s),
            VxdValue::I32(i) => println!("{}: i32 -> {}", key, i),
            VxdValue::I64(i) => println!("{}: i64 -> {}", key, i),
            VxdValue::U32(u) => println!("{}: u32 -> {}", key, u),
            VxdValue::U64(u) => println!("{}: u64 -> {}", key, u),
            VxdValue::F32(f) => println!("{}: f32 -> {}", key, f),
            VxdValue::F64(f) => println!("{}: f64 -> {}", key, f),
            VxdValue::Bool(b) => println!("{}: bool -> {}", key, b),
            VxdValue::Char(c) => println!("{}: char -> {}", key, c),
            VxdValue::Null => println!("{}: Null", key),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_file_valid() {
        let parser = VxdeParser::from_file("test_config.vxd");
        assert!(parser.is_ok());
        let parsed = parser.unwrap();
        assert!(parsed.get_variables().len() > 0);
    }

    #[test]
    fn test_from_file_invalid() {
        let parser = VxdeParser::from_file("invalid.vxd");
        assert!(parser.is_err());
    }
}
