//! This module provides a parser for `.vxd` files, extracting key-value pairs
//! based on their declared types. Supported types include string, integers (i32, i64),
//! unsigned integers (u32, u64), floating point (f32, f64), boolean, char, and null values.
//! The parser ensures that values are correctly parsed into the `VxdValue` enum and stored
//! in a hash map for further usage.

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{ self, BufRead };

/// Enum to represent different value types that can be parsed from a `.vxd` file.
/// The `Null` variant represents a missing or undefined value.
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
    Null, // Representing null values
}

impl VxdValue {
    //! Methods related to `VxdValue` can be added here for any additional functionalities.
    //! For now, it is just an enum for holding different value types.
}

/// A struct to hold the parsed key-value pairs from a `.vxd` file.
/// It stores the variables in a `HashMap` where keys are strings representing the variable names,
/// and values are of type `VxdValue` representing the parsed value.
pub struct VxdeParser {
    variables: HashMap<String, VxdValue>,
}

impl VxdeParser {
    /// Reads a `.vxd` file and parses its content into key-value pairs.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the `.vxd` file to be parsed.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` containing the parsed `VxdeParser` with key-value pairs if parsing succeeds.
    /// * `Err(io::Error)` if there is an issue reading the file or parsing its contents.
    ///
    /// # Example
    ///
    /// ```rust
    /// let parser = VxdeParser::from_file("config.vxd");
    /// match parser {
    ///     Ok(p) => println!("{:?}", p.get_variables()),
    ///     Err(e) => eprintln!("Error: {}", e),
    /// }
    /// ```
    pub fn from_file(file_path: &str) -> io::Result<Self> {
        let mut variables = HashMap::new();

        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);

        // Regex for matching valid variable declarations
        let re = Regex::new(r"(?m)^\s*([A-Za-z_][A-Za-z0-9_]*)\s*:\s*(string|i32|i64|u32|u64|f32|f64|bool|char)\s*(=\s*([^;]*))?\s*;").unwrap();

        // Iterate through the lines in the file
        for line in reader.lines() {
            let line = line?;

            // Check if the line matches the pattern of a valid declaration
            for caps in re.captures_iter(&line) {
                let name = &caps[1];
                let vtype = &caps[2];
                let value = caps.get(4).map_or("", |m| m.as_str()).trim();

                let parsed_value = match vtype {
                    "string" => {
                        if value == "null" || value.is_empty() {
                            VxdValue::Null
                        } else {
                            VxdValue::String(value.to_string())
                        }
                    },
                    "i32" => {
                        if value == "null" || value.is_empty() {
                            VxdValue::Null
                        } else {
                            value.parse::<i32>().ok().map(VxdValue::I32).unwrap_or(VxdValue::Null)
                        }
                    },
                    "i64" => {
                        if value == "null" || value.is_empty() {
                            VxdValue::Null
                        } else {
                            value.parse::<i64>().ok().map(VxdValue::I64).unwrap_or(VxdValue::Null)
                        }
                    },
                    "u32" => {
                        if value == "null" || value.is_empty() {
                            VxdValue::Null
                        } else {
                            value.parse::<u32>().ok().map(VxdValue::U32).unwrap_or(VxdValue::Null)
                        }
                    },
                    "u64" => {
                        if value == "null" || value.is_empty() {
                            VxdValue::Null
                        } else {
                            value.parse::<u64>().ok().map(VxdValue::U64).unwrap_or(VxdValue::Null)
                        }
                    },
                    "f32" => {
                        if value == "null" || value.is_empty() {
                            VxdValue::Null
                        } else {
                            value.parse::<f32>().ok().map(VxdValue::F32).unwrap_or(VxdValue::Null)
                        }
                    },
                    "f64" => {
                        if value == "null" || value.is_empty() {
                            VxdValue::Null
                        } else {
                            value.parse::<f64>().ok().map(VxdValue::F64).unwrap_or(VxdValue::Null)
                        }
                    },
                    "bool" => {
                        if value == "null" || value.is_empty() {
                            VxdValue::Null
                        } else {
                            value.parse::<bool>().ok().map(VxdValue::Bool).unwrap_or(VxdValue::Null)
                        }
                    },
                    "char" => {
                        if value == "null" || value.is_empty() {
                            VxdValue::Null
                        } else {
                            value.chars().next().map(VxdValue::Char).unwrap_or(VxdValue::Null)
                        }
                    },
                    _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Unsupported type: {}", vtype))),
                };

                variables.insert(name.to_string(), parsed_value);
            }
        }

        Ok(VxdeParser { variables })
    }

    /// Returns the parsed variables stored in the `variables` HashMap.
    ///
    /// # Returns
    ///
    /// * A reference to the `variables` HashMap containing parsed key-value pairs.
    ///
    /// # Example
    ///
    /// ```rust
    /// let variables = parser.get_variables();
    /// ```
    pub fn get_variables(&self) -> &HashMap<String, VxdValue> {
        &self.variables
    }
}

/// Function to demonstrate how to print the parsed variables from a `.vxd` file.
///
/// # Arguments
///
/// * `variables` - A reference to a `HashMap<String, VxdValue>` containing the parsed variables.
///
/// # Example
///
/// ```rust
/// print_variables(&parser.get_variables());
/// ```
pub fn print_variables(variables: &HashMap<String, VxdValue>) {
    // Iterate over the parsed variables and print them based on their type
    for (name, value) in variables {
        match value {
            VxdValue::String(val) => println!("{}: String = {}", name, val),
            VxdValue::I32(val) => println!("{}: i32 = {}", name, val),
            VxdValue::I64(val) => println!("{}: i64 = {}", name, val),
            VxdValue::U32(val) => println!("{}: u32 = {}", name, val),
            VxdValue::U64(val) => println!("{}: u64 = {}", name, val),
            VxdValue::F32(val) => println!("{}: f32 = {}", name, val),
            VxdValue::F64(val) => println!("{}: f64 = {}", name, val),
            VxdValue::Bool(val) => println!("{}: bool = {}", name, val),
            VxdValue::Char(val) => println!("{}: char = {}", name, val),
            VxdValue::Null => println!("{}: null", name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test for valid parsing of a `.vxd` file.
    /// This test will log any parsing errors encountered.
    #[test]
    fn test_valid_parsing() {
        let file_path = "tests/assets/valid_sample.vxd";
        let parser_result = VxdeParser::from_file(file_path);
    
        match parser_result {
            Ok(parser) => {
                let variables = parser.get_variables();
    
                // Debugging the actual value of USER_PASS to ensure no extra quotes
                if let Some(user_pass) = variables.get("USER_PASS") {
                    println!("Parsed USER_PASS: {:?}", user_pass);
                }
    
                assert!(variables.contains_key("USER_PASS"), "Missing key 'USER_PASS'");
                assert_eq!(variables["USER_PASS"], VxdValue::String("dhhdhdhd".to_string()), "Incorrect value for 'USER_PASS'");
                
                // Additional checks...
            }
            Err(e) => {
                panic!("Failed to parse the valid file '{}'. Error: {:?}", file_path, e);
            }
        }
    }
    

    /// Test for invalid parsing of a `.vxd` file.
    /// This test will log the error if the file parsing fails.
    #[test]
    fn test_invalid_parsing() {
        let file_path = "tests/assets/invalid_sample.vxd";
    
        // Try to parse the file and handle any errors gracefully
        let parser_result = VxdeParser::from_file(file_path);
    
        // Expect the file to fail to parse, so we assert the error
        match parser_result {
            Ok(_) => {
                panic!("Parsing should have failed for the invalid file '{}', but it succeeded.", file_path);
            }
            Err(e) => {
                // If the parsing fails as expected, log the error
                println!("Correctly failed to parse the invalid file '{}'. Error: {:?}", file_path, e);
            }
        }
    }
    
}
