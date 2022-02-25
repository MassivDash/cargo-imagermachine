extern crate termion;

use std::collections::HashMap;
use std::path::PathBuf;
mod errors;
use errors::no_image_files_error;
mod files;
use crate::files::{display_table, get_files_info};
use oxipng::{optimize, InFile, Options as OxiPngOptions, OutFile};
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

    for (path, name, _, _, _) in dir_files {
        println!("{}", name);

        let output_path = format!("{}/{}", config.get("output_path").unwrap(), name);
        let default_png_options = OxiPngOptions::default();
        let path_to_file: InFile = path.to_string().into();
        let path_to_output: OutFile = OutFile::Path(Some(PathBuf::from(&output_path)));
        println!("Working with {}", name);
        let file = optimize(&path_to_file, &path_to_output, &default_png_options);
        // let file = api::OptJob::open(path);
        match file {
            Ok(_) => {
                println!("Success 👁️‍🗨️ : Written to file {}", &output_path);
            }
            Err(error) => println!("{:#?}", error),
        }
    }

    println!("{}", options);
    println!("{:?}", config);
}
