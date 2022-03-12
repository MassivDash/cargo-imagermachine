use std::fs;

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
