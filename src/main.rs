extern crate termion;

mod display;
mod operations;
mod questions;
mod steps;

use steps::compare_files;

use crate::steps::{inspect_files, optimize_files, select_files, select_options};

fn main() {
    // Step 1: Select the user's files, get files and config
    let (dir_files, mut config) = select_files();

    // Step 2: Inspect the files
    inspect_files(&dir_files);

    // Step 3: Select the user's options
    select_options(&mut config);

    // Step 4: Optimize the files
    optimize_files(&dir_files, &mut config);

    // Step 5: Compare the files
    compare_files(&dir_files, &mut config);
}
