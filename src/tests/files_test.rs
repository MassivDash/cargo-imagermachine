#[cfg(test)]
use crate::operations::files::{compare_file_sizes, get_files_info, output_dir_check, FileInfo};
#[cfg(test)]
use crate::operations::optimize::{optimize_jpeg_image, optimize_png_image};

#[test]
fn check_file_collection() {
    let input_path = "./test/";
    let files = get_files_info(input_path);

    assert_eq!(files.len(), 4);
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
        mime_type: mime::IMAGE_PNG,
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
fn check_jpeg_optimization() {
    let output_dir = "./output/".to_string();

    // Delete output directory if it exists, so we have consistent results
    let path = "./test/test3.jpg".to_string();
    let name = "test3.jpg".to_string();

    output_dir_check(&output_dir);
    optimize_jpeg_image(&path, &name, &output_dir);

    // collect the transformed file and compare it to the original
    let output_files = get_files_info(&output_dir);
    let input_files = get_files_info(&"./test/");

    let input_file = input_files
        .iter()
        .filter(|x| x.path == "./test/test3.jpg")
        .next()
        .unwrap();

    let output_file = output_files
        .iter()
        .filter(|x| x.path == "./output/test3.jpg")
        .next()
        .unwrap();

    assert!(output_file.size_bytes < input_file.size_bytes);
}

#[test]
fn check_png_optimization() {
    let output_dir = "./output/".to_string();
    // Delete output directory if it exists, so we have consistent results
    let path = "./test/test2.png".to_string();
    let name = "test2.png".to_string();

    output_dir_check(&output_dir);
    optimize_png_image(&path, &name, &output_dir);

    // collect the transformed file and compare it to the original
    let output_files = get_files_info(&output_dir);
    let input_files = get_files_info(&"./test/");

    let input_file = input_files
        .iter()
        .filter(|x| x.path == "./test/test2.png")
        .next()
        .unwrap();

    let output_file = output_files
        .iter()
        .filter(|x| x.path == "./output/test2.png")
        .next()
        .unwrap();

    assert!(output_file.size_bytes < input_file.size_bytes);
}

#[test]
fn check_compare_size() {
    let input_path = "./test/";
    let files = get_files_info(input_path);
    let vec_files = files.iter().collect::<Vec<&FileInfo>>();

    let file_1: &FileInfo = vec_files
        .iter()
        .filter(|x| x.path == "./test/test2.png")
        .next()
        .unwrap();
    let file_2: &FileInfo = vec_files
        .iter()
        .filter(|x| x.path == "./test/test1.png")
        .next()
        .unwrap();
    let difference = compare_file_sizes(file_1, file_2);
    assert_eq!(difference.abs(), 327493);
}
