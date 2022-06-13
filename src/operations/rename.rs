use std::fs;

/// Rename files
/// # Arguments
/// * `path` - The path to the directory
/// * `npath` - The path to the directory
///
/// # Returns
/// * void  - No return value
/// * `std::io::Error` - If the directory does not exist
///         
/// # Example
/// ```
/// rename_files(&path, &npath);    
/// ```
/// # Errors
/// * `std::io::Error` - If the directory does not exist
///
pub fn rename_file(path: String, npath: String) -> std::io::Result<()> {
    if path != npath {
        let rn = fs::rename(path, npath);
        match rn {
            Ok(()) => return Ok(()),
            Err(e) => {
                println!("{:#?}", e);
                return Err(e);
            }
        }
    }
    return Ok(());
}
