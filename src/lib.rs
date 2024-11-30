use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

/// A struct to hold the parsed key-value pairs from a `.vxd` file.
pub struct VxdeParser {
    variables: HashMap<String, String>,
}

impl VxdeParser {
    /// Reads a `.vxd` file and parses its content into key-value pairs.
    pub fn from_file(file_path: &str) -> io::Result<Self> {
        let mut variables = HashMap::new();

        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let line = line?;

            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let pairs: Vec<&str> = line.split(';').collect();
            for pair in pairs {
                let pair = pair.trim();
                if pair.is_empty() {
                    continue;
                }

                let parts: Vec<&str> = pair.splitn(2, '=').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim().to_string();
                    let value = parts[1].trim().to_string();

                    let value = if value.starts_with('"') && value.ends_with('"') {
                        value[1..value.len()-1].to_string()
                    } else {
                        value
                    };

                    variables.insert(key, value);
                }
            }
        }

        Ok(VxdeParser { variables })
    }

    /// Returns the parsed variables.
    pub fn get_variables(&self) -> &HashMap<String, String> {
        &self.variables
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    // Helper function to create a temporary file for testing
    fn create_temp_file(content: &str) -> std::io::Result<String> {
        let temp_dir = std::env::temp_dir();
        let temp_file_path = temp_dir.join("test.vxd");
        let mut file = File::create(&temp_file_path)?;
        file.write_all(content.as_bytes())?;
        Ok(temp_file_path.to_str().unwrap().to_string())
    }

    #[test]
    fn test_basic_parsing() {
        let content = r#"
            USER_ISLOGGEDIN=true;USER_NAME="reanore";USER_PASS="1234";
        "#;
        let temp_file = create_temp_file(content).unwrap();

        let vxde = VxdeParser::from_file(&temp_file).unwrap();
        let variables = vxde.get_variables();

        assert_eq!(variables.get("USER_ISLOGGEDIN"), Some(&"true".to_string()));
        assert_eq!(variables.get("USER_NAME"), Some(&"reanore".to_string()));
        assert_eq!(variables.get("USER_PASS"), Some(&"1234".to_string()));

        fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_parsing_with_spaces() {
        let content = r#"
            USER_ISLOGGEDIN = true ;
            USER_NAME  = "reanore" ;
            USER_PASS = "1234" ;
        "#;
        let temp_file = create_temp_file(content).unwrap();

        let vxde = VxdeParser::from_file(&temp_file).unwrap();
        let variables = vxde.get_variables();

        assert_eq!(variables.get("USER_ISLOGGEDIN"), Some(&"true".to_string()));
        assert_eq!(variables.get("USER_NAME"), Some(&"reanore".to_string()));
        assert_eq!(variables.get("USER_PASS"), Some(&"1234".to_string()));

        fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_parsing_with_newlines() {
        let content = r#"
            USER_ISLOGGEDIN=true;

            USER_NAME="reanore";

            USER_PASS="1234";
        "#;
        let temp_file = create_temp_file(content).unwrap();

        let vxde = VxdeParser::from_file(&temp_file).unwrap();
        let variables = vxde.get_variables();

        assert_eq!(variables.get("USER_ISLOGGEDIN"), Some(&"true".to_string()));
        assert_eq!(variables.get("USER_NAME"), Some(&"reanore".to_string()));
        assert_eq!(variables.get("USER_PASS"), Some(&"1234".to_string()));

        fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_empty_file() {
        let content = "";
        let temp_file = create_temp_file(content).unwrap();

        let vxde = VxdeParser::from_file(&temp_file).unwrap();
        let variables = vxde.get_variables();

        assert!(variables.is_empty());

        fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_missing_values() {
        let content = r#"
            USER_ISLOGGEDIN=true;
            USER_NAME="reanore";
            USER_PASS;
        "#;
        let temp_file = create_temp_file(content).unwrap();

        let vxde = VxdeParser::from_file(&temp_file).unwrap();
        let variables = vxde.get_variables();

        assert_eq!(variables.get("USER_ISLOGGEDIN"), Some(&"true".to_string()));
        assert_eq!(variables.get("USER_NAME"), Some(&"reanore".to_string()));
        assert!(variables.get("USER_PASS").is_none());

        fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_invalid_format() {
        let content = r#"
            USER_ISLOGGEDIN true
            USER_NAME="reanore"
            USER_PASS=1234;
        "#;
        let temp_file = create_temp_file(content).unwrap();

        let vxde = VxdeParser::from_file(&temp_file).unwrap();
        let variables = vxde.get_variables();

        assert_eq!(variables.get("USER_ISLOGGEDIN"), None);
        assert_eq!(variables.get("USER_NAME"), Some(&"reanore".to_string()));
        assert_eq!(variables.get("USER_PASS"), Some(&"1234".to_string()));

        fs::remove_file(temp_file).unwrap();
    }
}

