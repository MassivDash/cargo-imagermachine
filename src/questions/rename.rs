use inquire::Confirm;

pub fn rename_files() -> bool {
    let ans = Confirm::new("rename files ?")
        .with_default(false)
        .with_help_message(
            "all files in the output folder will be renamed, newName_index.ext will be applied",
        )
        .prompt();

    match ans {
        Ok(true) => return true,
        Ok(false) => return false,
        Err(_) => return false,
    }
}
