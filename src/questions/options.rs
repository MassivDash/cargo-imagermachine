use crate::display::errors::generic_panic_error;
use inquire::Select;

pub fn get_options() -> String {
    // Ask for initial options questions;
    let options = vec!["default optimization", "resize and optimize", "custom job"];

    let ans = Select::new("What are we doing ?", options).prompt();

    return match ans {
        Ok(choice) => choice.to_string(),
        Err(error) => match error {
            _ => generic_panic_error(&error.to_string()),
        },
    };
}
