use crate::display::errors::generic_panic_error;
use inquire::CustomType;

pub fn get_resize_options() -> u64 {
    // Ask for resize options
    let amount = CustomType::<u64>::new("What is the target width?")
        .with_formatter(&|i| format!("${:.2}", i))
        .with_error_message("Please type a valid number")
        .with_help_message("Enter a number in pixels")
        .prompt();

    return match amount {
        Ok(value) => value,
        Err(err) => generic_panic_error(&err.to_string()),
    };
}
