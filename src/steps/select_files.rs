use std::collections::{HashMap, HashSet};

use crate::{
    display::errors::no_image_files_error,
    display::splash::{do_splash, step},
    operations::files::{get_files_info, FileInfo},
    questions::initial::get_initial,
};

pub struct FileConfig {
    pub input_path: String,
    pub output_path: String,
}

pub fn main() -> (HashSet<FileInfo>, FileConfig) {
    // BIG INTRO
    do_splash();

    step("Step 1: Select files 📁");
    let config: HashMap<&str, String> = get_initial();

    // Get files info based on input path
    let dir_files = get_files_info(config.get("input_path").unwrap());

    // We want to display error
    // if no images are found in the input folder
    no_image_files_error(&dir_files);

    // Return the files and the config
    let fileConfig = FileConfig {
        input_path: config.get("input_path").unwrap().to_string(),
        output_path: config.get("output_path").unwrap().to_string(),
    };

    return (dir_files, fileConfig);
}
