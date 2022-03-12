#[cfg(test)]
use crate::operations::files::{compare_file_sizes, get_files_info, FileInfo};

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
