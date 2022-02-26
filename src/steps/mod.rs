use crate::operations::files::FileInfo;
use std::collections::{HashMap, HashSet};

pub mod compare_files;
pub mod inspect_files;
pub mod optimize_files;
pub mod select_files;
pub mod select_options;

pub fn select_files() -> (HashSet<FileInfo>, HashMap<&'static str, String>) {
    return select_files::main();
}

pub fn inspect_files(dir_files: &HashSet<FileInfo>) -> () {
    return inspect_files::main(dir_files);
}

pub fn select_options(config: &mut HashMap<&str, String>) -> () {
    return select_options::main(config);
}

pub fn optimize_files(dir_files: &HashSet<FileInfo>, config: &mut HashMap<&str, String>) -> () {
    return optimize_files::main(dir_files, config);
}

pub fn compare_files(dir_files: &HashSet<FileInfo>, config: &mut HashMap<&str, String>) -> () {
    return compare_files::main(dir_files, config);
}
