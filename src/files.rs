use image::io::Reader;
use mime::Mime;
use prettytable::{cell, row, Table};
use size_format::SizeFormatterBinary;
use std::{
    collections::HashSet,
    fs::{self},
    path::Path,
};

use crate::errors::directory_error;

fn find_mimetype(filename: &String) -> Mime {
    let parts: Vec<&str> = filename.split('.').collect();

    let res = match parts.last() {
        Some(v) => match *v {
            "png" => mime::IMAGE_PNG,
            "jpg" => mime::IMAGE_JPEG,
            "json" => mime::APPLICATION_JSON,
            &_ => mime::TEXT_PLAIN,
        },
        None => mime::TEXT_PLAIN,
    };
    return res;
}

pub fn output_dir_check(path: &String) {
    let path = Path::new(path);
    if path.is_dir() {
        return;
    }
    let dir = fs::create_dir_all(path);
    if dir.is_err() {
        directory_error();
    }
    return;
}

fn file_size(path: &String) -> Result<u64, std::io::Error> {
    let file_size = fs::metadata(path)?.len();
    Ok(file_size)
}

fn file_resolution(path: &String) -> Result<String, std::io::Error> {
    let image = Reader::open(path)?.into_dimensions();
    let (width, height) = image.ok().unwrap();
    Ok(format!("{}px x {}px", width, height))
}

// Tell compiler to derive those instances for us
#[derive(PartialEq, Eq, Hash)]
pub struct FileInfo {
    pub path: String,
    pub name: String,
    pub size_formatted: String,
    pub size_bytes: u64,
    pub mime_type: String,
    pub resolution: String,
}

pub fn get_files_info(input_path: &str) -> HashSet<FileInfo> {
    let paths = fs::read_dir(input_path).unwrap().filter(|entry| {
        if entry.as_ref().unwrap().path().is_dir() {
            return false;
        }

        // Filter out files that are not images
        if entry.as_ref().unwrap().path().is_file() {
            let path = entry.as_ref().unwrap().path();
            let filename = path.file_name().unwrap().to_str().unwrap();
            let is_file = path.is_file();
            let is_image = filename.ends_with(".png") || filename.ends_with(".jpg");
            return is_image && is_file;
        } else {
            return false;
        }
    });

    let mut files = HashSet::new();

    for path in paths {
        let file = path.unwrap();
        let file_path = file.path().to_str().unwrap().to_string();
        let file_name = file.file_name().to_str().unwrap().to_string();
        let file_size = file_size(&file_path).unwrap();
        let file_size_formatted = SizeFormatterBinary::from(file_size).to_string() + "B";
        let file_type = find_mimetype(&file_name);
        let file_resolution = file_resolution(&file_path).unwrap();

        let file_info = FileInfo {
            path: file_path,
            name: file_name,
            size_formatted: file_size_formatted,
            size_bytes: file_size,
            mime_type: file_type.to_string(),
            resolution: file_resolution,
        };

        files.insert(file_info);
    }

    return files;
}

pub fn display_table(files: &HashSet<FileInfo>) {
    let mut table = Table::new();
    table.add_row(row![
        "Path",
        "Name",
        "Size",
        "Size bytes",
        "Type",
        "Resolution"
    ]);

    for file in files {
        table.add_row(row![
            file.path,
            file.name,
            file.size_formatted,
            file.mime_type,
            file.size_bytes,
            file.resolution
        ]);
    }

    table.printstd();
}
