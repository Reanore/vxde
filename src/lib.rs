use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{ self, BufRead };

/// Enum to represent different value types, including a null variant for the value.
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

/// A struct to hold the parsed key-value pairs from a `.vxd` file.
pub struct VxdeParser {
    variables: HashMap<String, VxdValue>,
}

impl VxdeParser {
    /// Reads a `.vxd` file and parses its content into key-value pairs.
    pub fn from_file(file_path: &str) -> io::Result<Self> {
        let mut variables = HashMap::new();

        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);

        // Regex for matching valid variable declarations
        let re = Regex::new(r"(?m)^\s*([A-Za-z_][A-Za-z0-9_]*)\s*:\s*(string|i32|i64|u32|u64|f32|f64|bool|char)\s*(=\s*([^;]*))?\s*;").unwrap();

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

    /// Returns the parsed variables.
    pub fn get_variables(&self) -> &HashMap<String, VxdValue> {
        &self.variables
    }
}

/// Function to demonstrate how to print the parsed variables.
pub fn print_variables(variables: &HashMap<String, VxdValue>) {
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

    #[test]
    fn test_valid_parsing() {
        let parser = VxdeParser::from_file("tests/assets/valid_sample.vxd");
        assert!(parser.is_ok());

        let parser = parser.unwrap();
        let variables = parser.get_variables();

        assert!(variables.contains_key("USER_PASS"));
        assert_eq!(variables["USER_PASS"], VxdValue::String("dhhdhdhd".to_string()));

        assert!(variables.contains_key("id_lowercase_name"));
        assert_eq!(variables["id_lowercase_name"], VxdValue::I32(323));

        assert!(variables.contains_key("NEXT_VAR_NOLINEBREAK"));
        assert_eq!(variables["NEXT_VAR_NOLINEBREAK"], VxdValue::String("hello".to_string()));

        assert!(variables.contains_key("EMPTY_VAR_WILL_BE_IGNORED"));
        assert_eq!(variables["EMPTY_VAR_WILL_BE_IGNORED"], VxdValue::Null); // Null value

        assert!(variables.contains_key("ANOTHER_ONE"));
    }

    #[test]
    fn test_invalid_parsing() {
        let parser = VxdeParser::from_file("tests/assets/invalid_sample.vxd");
        assert!(parser.is_err());
    }
}

