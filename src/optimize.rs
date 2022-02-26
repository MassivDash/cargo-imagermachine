use oxipng::{optimize, InFile, Options as OxiPngOptions, OutFile, PngError};
use std::path::PathBuf;

pub fn optimize_image(path: &String, name: &String, output_dir: &String) -> Result<(), PngError> {
    let output_path = format!("{}/{}", output_dir, name);
    let default_png_options = OxiPngOptions::default();
    let path_to_file: InFile = path.to_string().into();
    let path_to_output: OutFile = OutFile::Path(Some(PathBuf::from(&output_path)));

    // Optimize image TODO : Write codecs to optimize
    let file = optimize(&path_to_file, &path_to_output, &default_png_options);

    return file;
}
