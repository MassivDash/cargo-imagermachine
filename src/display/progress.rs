use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashSet;

use crate::operations::files::FileInfo;

pub fn progress_bar(dir_files: &HashSet<FileInfo>) -> ProgressBar {
    let progress_bar = ProgressBar::new(dir_files.len() as u64);
    progress_bar.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
        )
        .unwrap(),
    );

    return progress_bar;
}
