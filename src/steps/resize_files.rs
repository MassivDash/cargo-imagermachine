use std::collections::HashSet;

use crate::{
    display::splash::{hr, spacer, step},
    operations::files::FileInfo,
    questions::resize::get_resize_options,
};

pub fn main(dir_files: &HashSet<FileInfo>) -> () {
    hr();
    step("resizing files 🔍");
    get_resize_options();
    spacer();
    hr();
}
