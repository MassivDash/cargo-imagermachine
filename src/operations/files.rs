use image::io::Reader;
use mime::Mime;
use size_format::SizeFormatterBinary;
use std::{
    collections::HashSet,
    fs::{self},
    path::Path,
};

use crate::display::errors::directory_error;

fn find_mimetype(filename: &String) -> Mime {
    let parts: Vec<&str> = filename.split('.').collect();

    let res = match parts.last() {
        Some(v) => match *v {
            "png" => mime::IMAGE_PNG,
            "jpg" => mime::IMAGE_JPEG,
            "jpeg" => mime::IMAGE_JPEG,
            "json" => mime::APPLICATION_JSON,
            "webp" => mime::IMAGE_STAR,
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
    match image {
        Ok((width, height)) => Ok(format!("{}px x {}px", width, height)),
        Err(_) => Ok("Err".to_string()),
    }
}

// Tell compiler to derive those instances for us
#[derive(PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct FileInfo {
    pub path: String,
    pub name: String,
    pub size_formatted: String,
    pub size_bytes: u64,
    pub mime_type: Mime,
    pub resolution: String,
}

/// This function will return a list of files in a directory
///
/// # Arguments
/// * `path` - The path to the directory
///
/// # Returns
/// * `Vec<FileInfo>` - A list of files in the directory
/// # Example
/// ```
/// let files = get_files_info(&path);
/// ```
/// # Errors
/// * `std::io::Error` - If the directory does not exist
///
/// # Panics
/// * `std::io::Error` - If the directory does not exist
///
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
            let is_image = filename.ends_with(".png")
                || filename.ends_with(".jpg")
                || filename.ends_with(".jpeg")
                || filename.ends_with(".webp");
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
            mime_type: file_type,
            resolution: file_resolution,
        };

        files.insert(file_info);
    }

    return files;
}

/// This function compares 2 FileInfo structs and returns a difference in size (i128)
/// # Arguments
/// * `file1` - The first file to compare
/// * `file2` - The second file to compare
/// # Returns
/// * `i128` - The difference in size between the two files
/// # Example
/// ```
/// let difference = compare_files_size(&file1, &file2);
/// ```

pub fn compare_file_sizes(output_file: &FileInfo, input_file: &FileInfo) -> i128 {
    let output_size = output_file.size_bytes;
    let input_size = input_file.size_bytes;
    let difference: i128 = i128::from(output_size) - i128::from(input_size);
    return difference;
}
