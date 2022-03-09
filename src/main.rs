extern crate termion;

mod display;
mod operations;
mod questions;
mod steps;

use crate::steps::{inspect_files, optimize_files, select_files, select_options};
use steps::{compare_files, resize_files, select_options::Options};

pub struct Config {
    input_path: String,
    output_path: String,
    resize: bool,
    resize_width: u16,
    rename: bool,
    rename_name: String,
}

fn main() {
    // Step 1: Select the user's files, get files and config
    // input path and output path

    let mut config = Config {
        input_path: String::from(""),
        output_path: String::from(""),
        resize: false,
        resize_width: 0,
        rename: false,
        rename_name: String::from(""),
    };

    let (dir_files, file_config) = select_files();

    //Add file config to main config
    config.input_path = file_config.input_path;
    config.output_path = file_config.output_path;

    // Step 2: Inspect the files
    // Panic if no files are found
    inspect_files(&dir_files);

    // Step 3: Select the user's options
    let select_options = select_options();

    match select_options {
        Options::Default => {
            // Step 4: Optimize the files
            optimize_files(&dir_files, &config);
        }
        Options::Resize => {
            // Step 4: Resize the files
            resize_files(&dir_files);
            optimize_files(&dir_files, &config);
        }
        Options::Custom => {
            // Custom steps
            panic!("Not implemented yet");
        }
    }

    // Step 5: Compare the files
    compare_files(dir_files, config);
}
