use image::{imageops::FilterType, open, ImageError};

/// Resize an image to a given width.
///
/// # Arguments
/// * `path` - The path to the image
/// * `buffer_path` - The path to the output image
/// * `nwidth` - The width to resize the image to
///
/// # Returns
/// * `void` - No return value
///
/// # Example
/// ```
/// resize_image(&path, &buffer_path, nwidth);
/// ```
/// # Errors
/// * `std::io::Error` - If the image does not exist
/// * `image::ImageError` - Image error on save
///
/// # Panics
/// * `std::io::Error` - If the image does not exist
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
