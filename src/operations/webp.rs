use image::ImageReader;
use webp::{Encoder, WebPMemory};

use std::fs::write;

pub fn image_to_webp(path: &String, buffer_path: &String) -> Result<(), std::io::Error> {
    // Open path as DynamicImage
    let image = ImageReader::open(&path)
        .unwrap()
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    // Make webp::Encoder from DynamicImage.
    let encoder: Encoder = Encoder::from_image(&image).unwrap();

    // Encode image into WebPMemory.
    let encoded_webp: WebPMemory = encoder.encode(65f32);

    // Save webp image to file
    let save = write(&buffer_path, &*encoded_webp);
    return save;
}

pub fn convert_to_webp(path: &String, buffer_path: &String) {
    let save = image_to_webp(&path, &buffer_path);
    match save {
        Ok(_path) => {} // do nothing if successful,
        Err(e) => println!("{}", e),
    }
}
