use image::{imageops::FilterType, open, ImageError};

pub fn resize_image(path: &String, buffer_path: &String, nwidth: &u32) -> Result<(), ImageError> {
    let new_file = open(path).unwrap();
    let width = new_file.width();
    let height = new_file.height();
    let nheight = height * nwidth / width;

    let save = new_file
        .resize(*nwidth, nheight, FilterType::Nearest)
        .save_with_format(&buffer_path, image::ImageFormat::Png);
    return save;
}
