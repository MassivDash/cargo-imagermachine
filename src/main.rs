extern crate termion;

use std::collections::HashMap;

mod errors;
use errors::no_image_files_error;
mod files;
use crate::files::{display_table, get_files_info};
mod questions;
use questions::initial::get_initial;
use questions::options::get_options;
mod splash;
use splash::{do_splash, hr, spacer, step};

fn main() {
    // BIG INTRO
    do_splash();

    // Ask for inout and output paths
    step("Step 1: Select files 📁");
    let mut config: HashMap<&str, String> = get_initial();

    // Get files info based on input path
    let dir_files = get_files_info(config.get("input_path").unwrap());

    // If no image files found, exit
    no_image_files_error(&dir_files);

    // Separator for table
    hr();
    step("Step 2: Inspect files 🔍");
    display_table(&dir_files);
    spacer();
    hr();

    // Ask for output questions;
    step("Step 3: Select options 🏷️");

    let options = get_options();
    config.insert("options", options.to_string());

    println!("{}", options);
    println!("{:?}", config);
}
