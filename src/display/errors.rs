use crate::operations::files::FileInfo;
use std::collections::HashSet;
use termion::{color, style};

pub fn no_image_files_error(dir_files: &HashSet<FileInfo>) {
    if dir_files.len() == 0 {
        big_error();
        println!("{}", color::Fg(color::Blue));
        println!(
            "{}Error: Rust is about to go into panic!{}",
            style::Bold,
            color::Fg(color::Red)
        );
        println!("Error: No image files found in the directory");
        println!("{}", color::Fg(color::Reset));
        panic!("No image files found in the directory");
    }
}

pub fn directory_error() {
    big_error();
    println!("{}", color::Fg(color::Blue));
    println!(
        "{}Error: Rust is about to go into panic!{}",
        style::Bold,
        color::Fg(color::Red)
    );
    println!("Error: Directory does not exist and cannot be created");
    println!("{}", color::Fg(color::Reset));
    panic!("Directory does not exist and cannot be created");
}

pub fn big_error() {
    println!();
    println!(
        "{}
    .d88b. 888d888888d888 .d88b. 888d888 
    d8P  Y8b888P   888P   d88  88b888P   
    88888888888    888    888  888888     
    Y8b.    888    888    Y88..88P888     
      Y8888 888    888      Y88P  888       
",
        color::Fg(color::Red)
    );
    println!();
    println!();
}

pub fn generic_panic_error(error: &str) -> ! {
    big_error();
    println!("{}", color::Fg(color::Blue));
    println!(
        "{}Error: Rust is about to go into panic!{}",
        style::Bold,
        color::Fg(color::Red)
    );
    println!("Error: {:#?}", error);
    println!("{}", color::Fg(color::Reset));
    panic!("{:#?}", error);
}

pub fn optimize_error(error: String) {
    big_error();
    println!("{}", color::Fg(color::Blue));
    println!("{}Error!{}", style::Bold, color::Fg(color::Red));
    println!("Error: {:#?}", error);
    println!("{}", color::Fg(color::Reset));
}
