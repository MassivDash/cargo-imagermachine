use crate::display::errors::generic_panic_error;
use inquire::Text;
use std::collections::HashMap;

pub fn get_initial() -> HashMap<&'static str, String> {
    let mut config = HashMap::new();

    let input_path = Text::new("Path")
        .with_default("./test")
        .with_help_message("Enter directory name (default is current folder")
        .prompt();

    println!();

    match input_path {
        Ok(path) => config.insert("input_path", path),
        Err(error) => match error {
            _ => {
                generic_panic_error(&error.to_string());
            }
        },
    };

    let output_path = Text::new("Output Path")
        .with_default("./output")
        .with_help_message("Enter output directory path, default /output")
        .prompt();

    println!();

    match output_path {
        Ok(path) => config.insert("output_path", path),
        Err(error) => match error {
            _ => {
                println!("Error: {}", error);
                return config;
            }
        },
    };
    return config;
}
