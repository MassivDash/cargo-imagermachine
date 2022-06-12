use crate::{
    display::{
        progress::progress_bar,
        splash::{hr, spacer, step},
    },
    operations::{
        files::{output_dir_check, FileInfo},
        optimize::{execute_optimization},
    },
    Config,
};
use std::collections::HashSet;

pub fn main(dir_files: &HashSet<FileInfo>, config: &Config) -> () {
    step("Step 4: Optimizing files 🔨");
    println!();

    // Create progress bar
    let progress_bar = progress_bar(&dir_files);
    // Start ticking
    progress_bar.tick();
    // Create output directory if it doesn't exist
    // Panic if permission denied or other error
    output_dir_check(&config.output_path);

    for (i, file_info) in dir_files.iter().enumerate() {
        // Optimize image TODO : Write codecs to optimize

        let file = execute_optimization(&file_info.path, &file_info.name, file_info.mime_type.clone(), &config.output_path);
        // Update progress bar after each optimization
        match file {
            true => {
                if i == dir_files.len() - 1 {
                    progress_bar.finish();
                } else {
                    progress_bar.inc(1);
                }
            }
            false => panic!("Error optimizing file"),
        }
    }
    spacer();
    println!("Success 🚀 : All files optimized");
    spacer();
    hr();
    println!();
}
