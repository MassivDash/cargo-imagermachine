extern crate termion;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;

mod errors;
mod files;
mod optimize;
mod questions;
mod splash;
mod tables;

use crate::{
    errors::no_image_files_error,
    files::{compare_file_sizes, get_files_info, output_dir_check, FileInfo},
    optimize::optimize_image,
    questions::compare::compare_results,
    questions::initial::get_initial,
    questions::options::get_options,
    splash::{do_splash, hr, spacer, step},
    tables::display_files_table,
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
    display_files_table(&dir_files);
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

    // Create output directory if it doesn't exist
    // Panic if permission denied or other error
    output_dir_check(config.get("output_path").unwrap());

    for (i, file_info) in dir_files.iter().enumerate() {
        // Optimize image TODO : Write codecs to optimize
        let file = optimize_image(
            &file_info.path,
            &file_info.name,
            config.get("output_path").unwrap(),
        );

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
    println!("Success 🚀 : All files optimized");
    spacer();
    hr();
    println!();
    step("Step 5: Compare results 📊");
    spacer();

    let start_compare = compare_results();

    if start_compare {
        step("Step 5: Comparing files 🔍");
        println!();
        let files_info = get_files_info(config.get("output_path").unwrap());
        for output_file in files_info {
            let filter_files = &dir_files
                .iter()
                .filter(|file| file.name == output_file.name)
                .collect::<Vec<&FileInfo>>();

            let diff = compare_file_sizes(&output_file, &filter_files.get(0).unwrap());
            println!("{}", diff);
        }
    }

    hr();
    spacer();
    println!("Thanks for using this tool 🙏");
    // println!("{}", options);
    // println!("{:?}", config);
}
