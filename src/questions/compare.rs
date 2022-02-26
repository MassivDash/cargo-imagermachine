use inquire::Confirm;

pub fn compare_results() -> bool {
    let ans = Confirm::new("Compare results ?")
        .with_default(false)
        .with_help_message("display the difference between the images")
        .prompt();

    match ans {
        Ok(true) => return true,
        Ok(false) => return false,
        Err(_) => return false,
    }
}
