use image::io::Reader;
use mime::Mime;
use prettytable::{cell, row, Table};
use size_format::SizeFormatterBinary;
use std::{
    collections::HashSet,
    fs::{self},
};

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

fn file_size(path: &String) -> Result<u64, std::io::Error> {
    let file_size = fs::metadata(path)?.len();
    Ok(file_size)
}

fn file_resolution(path: &String) -> Result<String, std::io::Error> {
    let image = Reader::open(path)?.into_dimensions();
    let (width, height) = image.ok().unwrap();
    Ok(format!("{}px x {}px", width, height))
}

pub fn get_files_info(input_path: &str) -> HashSet<(String, String, String, Mime, String)> {
    let mut files = HashSet::new();

    let paths = fs::read_dir(input_path).unwrap().filter(|entry| {
        if entry.as_ref().unwrap().path().is_dir() {
            return false;
        }

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

    for path in paths {
        let file = path.unwrap();
        let file_path = file.path().to_str().unwrap().to_string();
        let file_name = file.file_name().to_str().unwrap().to_string();
        let file_size = file_size(&file_path).unwrap();
        let file_size_formatted = SizeFormatterBinary::from(file_size).to_string() + "B";
        let file_type = find_mimetype(&file_name);
        let file_resolution = file_resolution(&file_path).unwrap();

        files.insert((
            file_path,
            file_name,
            file_size_formatted,
            file_type,
            file_resolution,
        ));
    }

    return files;
}

pub fn display_table(files: &HashSet<(String, String, String, Mime, String)>) {
    let mut table = Table::new();
    table.add_row(row!["Path", "Name", "Size", "Type", "Resolution"]);

    for file in files {
        table.add_row(row![file.0, file.1, file.2, file.3, file.4]);
    }

    table.printstd();
}
