#[cfg(test)]
use crate::operations::files::{get_files_info, output_dir_check, FileInfo};
#[cfg(test)]
use crate::operations::webp::convert_to_webp;

#[test]
fn check_image_to_webp() {
    let output_dir = "./output/".to_string();
    // Delete output directory if it exists, so we have consistent results
    let path = "./test/test2.png".to_string();
    let buffer_path = "./output/test2.webp".to_string();
    output_dir_check(&output_dir);
    convert_to_webp(&path, &buffer_path);

    // collect the transformed file and compare it to the original
    let output_files = get_files_info(&output_dir);
    let input_files = get_files_info(&"./test/");

    let input_file = input_files
        .iter()
        .filter(|x| x.path == "./test/test2.png")
        .next()
        .unwrap();

    let output_file: &FileInfo = output_files
        .iter()
        .filter(|x| x.path == "./output/test2.webp")
        .next()
        .unwrap();

    assert!(output_file.size_bytes < input_file.size_bytes);
}
