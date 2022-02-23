use std::collections::HashMap;
extern crate termion;
use crate::files::{display_table, get_files_info};
use inquire::Select;
mod errors;
mod files;
mod questions;
mod splash;

fn main() {
    // BIG INTRO
    splash::do_splash();

    let mut init_questions_config: HashMap<&str, String> = questions::initial::get_initial();

    let dir_files = get_files_info(init_questions_config.get("input_path").unwrap());

    errors::no_image_files_error(&dir_files);
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
