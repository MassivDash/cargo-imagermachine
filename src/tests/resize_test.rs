#[cfg(test)]
use crate::operations::files::{get_files_info, output_dir_check, FileInfo};
#[cfg(test)]
use crate::operations::resize::resize_image;

#[test]
fn check_ratio() {
    let width: u32 = 1600;
    let height: u32 = 500;
    let nwidth: u32 = 500;

    fn ratio(width: u32, height: u32) -> f32 {
        let ratio = height as f32 / width as f32;
        ratio.round()
    }

    fn resize(nwidth: u32, width: u32, height: u32) -> u32 {
        return height * nwidth / width;
    }
    let before_ration = ratio(width, height);
    let after_ration = ratio(nwidth, resize(nwidth, width, height));
    assert_eq!(before_ration, after_ration);
}

#[test]
fn check_resize_image() {
    let output_dir = "./output/".to_string();
    // Delete output directory if it exists, so we have consistent results
    let path = "./test/test2.png".to_string();
    let buffer_path = "./output/test2.png".to_string();
    output_dir_check(&output_dir);
    resize_image(&path, &buffer_path, &500).unwrap();

    // collect the transformed file and compare it to the original
    let output_files = get_files_info(&output_dir);

    let output_file: &FileInfo = output_files
        .iter()
        .filter(|x| x.path == "./output/test2.png")
        .next()
        .unwrap();

    assert!(output_file.resolution == "499px x 389px");
}
