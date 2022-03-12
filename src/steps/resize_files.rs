use std::collections::HashSet;

use crate::{
    display::{
        errors::optimize_error,
        progress::progress_bar,
        splash::{hr, spacer, step},
    },
    operations::{
        files::{output_dir_check, FileInfo},
        optimize::optimize_image,
        resize::resize_image,
    },
    questions::resize::get_resize_options,
    Config,
};

pub fn main(dir_files: &HashSet<FileInfo>, config: &Config) -> () {
    hr();
    step("resizing files 🔍");
    //Get new width from the user
    let nwidth = get_resize_options();

    // Create progress bar
    let progress_bar = progress_bar(&dir_files);
    // Start ticking
    progress_bar.tick();
    // Create output directory if it doesn't exist
    // Panic if permission denied or other error
    output_dir_check(&config.output_path);

    for (i, file) in dir_files.iter().enumerate() {
        // Resize image and put it in the output folder
        let buffer_path = format!("{}/{}", config.output_path, file.name);
        resize_image(&file.path, &buffer_path, &nwidth).unwrap();

        // Optimize / overwrite image directly in the output folder
        // So we leave the originals alone
        let file = optimize_image(&buffer_path, &file.name, &config.output_path);

        match file {
            Ok(_) => {
                // Update progress bar after each optimization
                if i == dir_files.len() - 1 {
                    progress_bar.finish();
                } else {
                    progress_bar.inc(1);
                }
            }
            Err(error) => optimize_error(error),
        }
    }

    spacer();
    hr();
}
