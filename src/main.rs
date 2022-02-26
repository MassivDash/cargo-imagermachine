extern crate termion;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;

mod errors;
mod files;
mod optimize;
mod questions;
mod splash;

use crate::{
    errors::no_image_files_error,
    files::{display_table, get_files_info},
    optimize::optimize_image,
    questions::initial::get_initial,
    questions::options::get_options,
    splash::{do_splash, hr, spacer, step},
};

fn main() {
    // BIG INTRO
    do_splash();

    // Ask for inout and output paths
    step("Step 1: Select files 📁");
    let mut config: HashMap<&str, String> = get_initial();

    // Get files info based on input path
    let dir_files = get_files_info(config.get("input_path").unwrap());

    // We want to display error
    // if no images are found in the input folder
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

    spacer();

    step("Step 4: Optimizing files 🔨");
    println!();
    let progress_bar = ProgressBar::new(dir_files.len() as u64);
    progress_bar.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
        )
        .unwrap(),
    );
    progress_bar.tick();
    for (i, (path, name, _, _, _)) in dir_files.iter().enumerate() {
        // Optimize image TODO : Write codecs to optimize
        let file = optimize_image(&path, &name, config.get("output_path").unwrap());

        // Update progress bar after each optimization
        match file {
            Ok(_) => {
                if i == dir_files.len() - 1 {
                    progress_bar.finish();
                } else {
                    progress_bar.inc(1);
                }
            }
            Err(error) => println!("{:#?}", error),
        }
    }
    spacer();
    println!("Success 👁️‍🗨️");

    // println!("{}", options);
    // println!("{:?}", config);
}
