use vxde::*;

#[test]
fn test_parsing_with_newlines() {
    let file_path = "tests/assets/parsing_tests.rs/test_parsing_with_newlines.vxd";

    let parser_result = VxdeParser::from_file(file_path);

    match parser_result {
        Ok(parser) => {
            let variables = parser.get_variables();

            if let Some(user_parr) = variables.get("USER_PARR") {
                println!("Parsed USER_PARR: {:?}", user_parr);
            }

            assert!(variables.contains_key("USER_PARR"), "Missing key 'USER_PARR'");
            assert_eq!(variables["USER_PARR"], VxdValue::String("reanore\nflkkllkkll".to_string()), "Incorrect value for 'USER_PARR'");

        }
        Err(e) => {
            panic!("Failed to parse the valid file '{}'. Error: {:?}", file_path, e);
        }
    }
}