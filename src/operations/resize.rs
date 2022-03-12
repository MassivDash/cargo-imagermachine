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

#[cfg(test)]
mod tests {
    #[test]
    fn check_ratio() {
        let width = 1600;
        let height = 500;

        let nwidth = 500;

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
}
