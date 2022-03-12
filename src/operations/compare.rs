use crate::operations::files::{compare_file_sizes, get_files_info, FileInfo};
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct FileReport {
    pub name: String,
    pub original_size: u64,
    pub original_formatted_size: String,
    pub optimized_formatted_size: String,
    pub resolution: String,
    pub size_diff: i128,
}

pub fn compare_files(files: HashSet<FileInfo>, output_dir: String) -> HashSet<FileReport> {
    let mut files_report: HashSet<FileReport> = HashSet::new();
    let files_info = get_files_info(&output_dir);
    for output_file in files_info {
        let filter_files = files
            .iter()
            .filter(|&file| {
                return file.name == output_file.name;
            })
            .collect::<Vec<&FileInfo>>();

        let found_item = filter_files.get(0);
        match found_item {
            Some(file) => {
                let original_formatted_size = file.size_formatted.clone().to_string();

                let diff = compare_file_sizes(&output_file, &file);
                let file_report = FileReport {
                    name: output_file.name,
                    original_size: file.size_bytes,
                    original_formatted_size: original_formatted_size,
                    optimized_formatted_size: output_file.size_formatted,
                    resolution: output_file.resolution,
                    size_diff: diff,
                };
                files_report.insert(file_report);
            }
            None => {}
        }
    }

    return files_report;
}
