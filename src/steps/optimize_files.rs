use crate::{
    display::{
        progress::progress_bar,
        splash::{hr, spacer, step},
    },
    operations::{
        files::{output_dir_check, FileInfo},
        optimize::optimize_image,
    },
};
use std::collections::{HashMap, HashSet};

pub fn main(dir_files: &HashSet<FileInfo>, config: &mut HashMap<&str, String>) -> () {
    step("Step 4: Optimizing files 🔨");
    println!();

    // Create progress bar
    let progress_bar = progress_bar(&dir_files);
    // Start ticking
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
}
