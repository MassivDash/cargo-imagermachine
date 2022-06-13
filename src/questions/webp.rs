use inquire::Confirm;

pub fn convert_files_to_webp() -> bool {
    let ans = Confirm::new("Convert to webp 🚀 ?")
        .with_default(false)
        .with_help_message("all files in the output folder will be converted to webp format")
        .prompt();

    match ans {
        Ok(true) => return true,
        Ok(false) => return false,
        Err(_) => return false,
    }
}
