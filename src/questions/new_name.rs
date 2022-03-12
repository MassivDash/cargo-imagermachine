use inquire::Text;

use crate::display::errors::generic_panic_error;

pub fn get_new_name() -> String {
    let new_name = Text::new("New Name")
        .with_default("newName")
        .with_help_message("Enter new name for the file")
        .prompt();

    match new_name {
        Ok(name) => return name,
        Err(error) => generic_panic_error(error.to_string().as_str()),
    }
}
