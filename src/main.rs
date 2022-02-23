use inquire::{Select, Text};
use std::collections::HashMap;

mod files;

fn main() {
    println!("
d8b                                                                         888     d8b                 
Y8P                                                                         888     Y8P                 
                                                                            888                         
88888888b.d88b.  8888b.  .d88b.  .d88b. 888d88888888b.d88b.  8888b.  .d8888b88888b. 88888888b.  .d88b.  
888888  888  88b     88bd88P88bd8P  Y8b888P  888  888  88b     88bd88P   888   88b888888   88bd8P  Y8b 
888888  888  888.d888888888  88888888888888    888  888  888.d888888888     888  888888888  88888888888 
888888  888  888888  888Y88b 888Y8b.    888    888  888  888888  888Y88b.   888  888888888  888Y8b.     
888888  888  888 Y888888  Y88888  Y8888 888    888  888  888 Y888888  Y8888P888  888888888  888  Y8888  
                             888                                                                        
                        Y8b d88P                                                                        
                          Y88P     
");

    let mut config = HashMap::new();

    let input_path = Text::new("Path")
        .with_default("./test")
        .with_help_message("Enter directory path (type . for current)")
        .prompt();

    match input_path {
        Ok(path) => config.insert("input_path", path),
        Err(error) => match error {
            _ => {
                println!("Error: {}", error);
                return;
            }
        },
    };

    let output_path = Text::new("Output Path")
        .with_default("./output")
        .with_help_message("Enter output directory path, default /output")
        .prompt();

    match output_path {
        Ok(path) => config.insert("output_path", path),
        Err(error) => match error {
            _ => {
                println!("Error: {}", error);
                return;
            }
        },
    };

    files::get_files_info(config.get("input_path").unwrap());

    let options = vec![
        "default optimization",
        "resize and optimize",
        "custom optimization",
        "add watermark",
    ];

    let ans = Select::new("What are we doing ?", options).prompt();

    match ans {
        Ok(choice) => config.insert("action_question", choice.to_string()),
        Err(error) => match error {
            _ => {
                println!("Error: {}", error);
                return;
            }
        },
    };
}
