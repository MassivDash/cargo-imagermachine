use mime::Mime;
use oxipng::{optimize, InFile, Options as OxiPngOptions, OutFile, PngError};
use std::{path::PathBuf};


pub fn optimize_png_image(path: &String, name: &String, output_dir: &String) -> bool {
    let output_path = format!("{}/{}", output_dir, name);
    let default_png_options = OxiPngOptions::default();
    let path_to_file: InFile = path.to_string().into();
    let path_to_output: OutFile = OutFile::Path(Some(PathBuf::from(&output_path)));

    // Optimize image TODO : Write codecs to optimize
    let file: Result<(), PngError> = optimize(&path_to_file, &path_to_output, &default_png_options);

    match file {
        Ok(_) => return true,
        Err(error) => panic!("{:#?}", error),
    }
}

pub fn optimize_jpeg_image(path: &String, name: &String, output_dir: &String) -> bool {
    let output_path = format!("{}/{}", output_dir, name);
    // read JPEG data from file
    let jpeg_data = std::fs::read(path).unwrap();

    // decompress `jpeg_data` into an `image::RgbImage`
    let image: image::RgbImage = turbojpeg::decompress_image(&jpeg_data).unwrap();

    // compress `image` into JPEG with quality 95 and 2x2 chrominance subsampling
    let jpeg_data = turbojpeg::compress_image(&image, 95, turbojpeg::Subsamp::Sub2x2).unwrap();

    let file = std::fs::write(output_path, jpeg_data);
    match file {
        Ok(_) => return true,
        Err(error) => panic!("{:#?}", error)
    }
        
}

pub fn execute_optimization(file_path: &String, file_name: &String, mime_type: Mime, output_dir: &String) -> bool {
    let file;
    if mime_type == mime::IMAGE_JPEG {
    file = optimize_jpeg_image(&file_path, &file_name, &output_dir);
    } else {
    file = optimize_png_image(&file_path, &file_name, &output_dir);
    }
    return file;
}