use prettytable::{cell, row, Table};

use size_format::SizeFormatterBinary;
use std::{
    collections::HashSet,
    fs::{self},
};

fn file_size(path: &String) -> Result<u64, std::io::Error> {
    let file_size = fs::metadata(path)?.len();
    Ok(file_size)
}

pub fn get_files_info(input_path: &str) -> HashSet<(String, String, String)> {
    let mut files = HashSet::new();

    let paths = fs::read_dir(input_path).unwrap();

    for path in paths {
        let file = path.unwrap();
        let file_path = file.path().to_str().unwrap().to_string();
        let file_name = file.file_name().to_str().unwrap().to_string();
        let file_size = file_size(&file_path).unwrap();
        let file_size_formatted = SizeFormatterBinary::from(file_size).to_string() + "B";
        files.insert((file_path, file_name, file_size_formatted));
    }

    return files;
}

pub fn display_table(files: &HashSet<(String, String, String)>) {
    let mut table = Table::new();
    table.add_row(row!["Path", "Name", "Size"]);

    for file in files {
        table.add_row(row![file.0, file.1, file.2]);
    }

    table.printstd();
}
