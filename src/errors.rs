use mime::Mime;
use std::collections::HashSet;

pub fn no_image_files_error(dir_files: &HashSet<(String, String, String, Mime, String)>) {
    if dir_files.len() == 0 {
        big_error();
        println!();
        println!("Error: Rust is about to go into panic!");
        println!("Error: No image files found in the directory");
        println!();
        panic!("No image files found in the directory");
    }
}

pub fn big_error() {
    println!();
    println!(
        "
    .d88b. 888d888888d888 .d88b. 888d888 
    d8P  Y8b888P   888P   d88  88b888P   
    88888888888    888    888  888888     
    Y8b.    888    888    Y88..88P888     
      Y8888 888    888      Y88P  888       
"
    );
    println!();
    println!();
}
