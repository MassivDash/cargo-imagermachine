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
#[derive(PartialEq, Eq, Hash, Debug)]
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

pub fn compare_file_sizes(output_file: &FileInfo, input_file: &FileInfo) -> i128 {
    let output_size = output_file.size_bytes;
    let input_size = input_file.size_bytes;
    let difference: i128 = i128::from(output_size) - i128::from(input_size);
    return difference;
}

#[cfg(test)]
mod tests {
    use crate::operations::files::get_files_info;

    use super::{compare_file_sizes, FileInfo};

    #[test]
    fn check_file_collection() {
        let input_path = "./test/";
        let files = get_files_info(input_path);

        assert_eq!(files.len(), 2);
    }

    #[test]
    fn check_file_info() {
        let input_path = "./test/";
        let files = get_files_info(input_path);

        let file = files
            .iter()
            .filter(|x| x.path == "./test/test2.png")
            .next()
            .unwrap();

        let file_info = FileInfo {
            path: "./test/test2.png".to_string(),
            name: "test2.png".to_string(),
            size_formatted: "124.2KiB".to_string(),
            size_bytes: 127264,
            mime_type: "image/png".to_string(),
            resolution: "986px x 768px".to_string(),
        };

        assert_eq!(file.path, file_info.path);
        assert_eq!(file.name, file_info.name);
        assert_eq!(file.size_formatted, file_info.size_formatted);
        assert_eq!(file.size_bytes, file_info.size_bytes);
        assert_eq!(file.mime_type, file_info.mime_type);
        assert_eq!(file.resolution, file_info.resolution);
    }

    #[test]
    fn check_compare_size() {
        let input_path = "./test/";
        let files = get_files_info(input_path);
        let vec_files = files.iter().collect::<Vec<&FileInfo>>();

        let file_1: &FileInfo = vec_files[0];
        let file_2: &FileInfo = vec_files[1];
        let difference = compare_file_sizes(file_1, file_2);
        assert_eq!(difference.abs(), 327493);
    }
}
