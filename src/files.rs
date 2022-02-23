use size_format::SizeFormatterBinary;
use std::fs::{self};

fn file_size(path: &str) -> Result<u64, std::io::Error> {
    let file_size = fs::metadata(path)?.len();
    Ok(file_size)
}

pub fn get_files_info(input_path: &str) {
    let paths: Vec<String> = fs::read_dir(input_path)
        .unwrap()
        .filter_map(|maybe_dir_entry| {
            let dir_entry = maybe_dir_entry.ok()?;
            let path_buf = dir_entry.path();
            let file_name = path_buf.file_name()?;
            let string = file_name.to_str()?;
            Some(string.to_string())
        })
        .collect();

    for path in paths {
        let file_size = file_size(&path).unwrap();
        println!(
            "{} {}",
            path,
            SizeFormatterBinary::from(file_size).to_string()
        );
    }
}
