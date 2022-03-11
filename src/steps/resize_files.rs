use image::open;
use std::collections::HashSet;

use crate::{
    display::{
        progress::progress_bar,
        splash::{hr, spacer, step},
    },
    operations::{
        files::{output_dir_check, FileInfo},
        optimize::optimize_image,
    },
    questions::resize::get_resize_options,
    Config,
};
use image::imageops::FilterType;

pub fn main(dir_files: &HashSet<FileInfo>, config: &Config) -> () {
    hr();
    step("resizing files 🔍");
    let nwidth = get_resize_options();

    // Create progress bar
    let progress_bar = progress_bar(&dir_files);
    // Start ticking
    progress_bar.tick();
    // Create output directory if it doesn't exist
    // Panic if permission denied or other error
    output_dir_check(&config.output_path);

    for (i, file) in dir_files.iter().enumerate() {
        let new_file = open(format!("{}", file.path)).unwrap();

        let width = new_file.width();
        let height = new_file.height();
        let nheight = height * nwidth / width;

        let buffer_path = format!("{}/{}", config.output_path, file.name);

        new_file
            .resize(nwidth, nheight, FilterType::Nearest)
            .save_with_format(&buffer_path, image::ImageFormat::Png)
            .unwrap();

        let file = optimize_image(&buffer_path, &file.name, &config.output_path);

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
    hr();
}
