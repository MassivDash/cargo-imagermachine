use std::collections::HashSet;

use crate::{
    display::splash::{hr, spacer, step},
    display::tables::display_files_table,
    operations::files::FileInfo,
};

pub fn main(dir_files: &HashSet<FileInfo>) -> () {
    hr();
    step("Step 2: Inspect files 🔍");
    display_files_table(dir_files);
    spacer();
    hr();
}
