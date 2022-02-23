use size_format::SizeFormatterBinary;
use std::fs::{self};

fn file_size(path: &str) -> Result<u64, std::io::Error> {
    let file_size = fs::metadata(path)?.len();
    Ok(file_size)
}

pub fn get_files_info(input_path: &str) {
    let paths = fs::read_dir(input_path);
    let file_paths: Vec<String> = paths
        .unwrap()
        .filter_map(|maybe_dir_entry| {
            let dir_entry = maybe_dir_entry.ok()?;
            let path_buf = dir_entry.path();
            let file_path = path_buf.to_str()?;
            Some(file_path.to_string())
        })
        .collect();
    println!("{:?}", file_paths);

    for path in file_paths {
        let file_size = file_size(&path).unwrap();
        println!(
            "{} {}B",
            path,
            SizeFormatterBinary::from(file_size).to_string()
        );
    }
}
