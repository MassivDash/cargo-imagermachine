use std::collections::HashMap;

use crate::files::{display_table, get_files_info};
use inquire::Select;
mod files;
mod questions;

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

    let mut init_questions_config: HashMap<&str, String> = questions::get_initial_config();

    let dir_files = get_files_info(init_questions_config.get("input_path").unwrap());

    if dir_files.len() == 0 {
        println!();
        println!("Error: Rust is about to go into panic!");
        println!("Error: No image files found in the directory");
        println!();
        panic!("No image files found in the directory");
    }

    display_table(&dir_files);

    let options = vec![
        "default optimization",
        "resize and optimize",
        "custom optimization",
        "add watermark",
    ];

    let ans = Select::new("What are we doing ?", options).prompt();

    match ans {
        Ok(choice) => init_questions_config.insert("action_question", choice.to_string()),
        Err(error) => match error {
            _ => {
                println!("Error: {}", error);
                return;
            }
        },
    };
}
