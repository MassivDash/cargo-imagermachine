use crate::operations::files::{compare_file_sizes, get_files_info, FileInfo};
use std::collections::HashSet;

pub fn compare_files(files: &HashSet<FileInfo>, output_dir: &String) -> () {
    let files_info = get_files_info(output_dir);
    for output_file in files_info {
        let filter_files = files
            .iter()
            .filter(|file| file.name == output_file.name)
            .collect::<Vec<&FileInfo>>();

        let diff = compare_file_sizes(&output_file, &filter_files.get(0).unwrap());
        println!("{}", diff);
    }
}
