use crate::operations::files::FileInfo;
use std::collections::HashSet;

use self::select_files::FileConfig;
use self::select_options::Options;
use crate::Config;

pub mod compare_files;
pub mod inspect_files;
pub mod optimize_files;
pub mod rename_files;
pub mod resize_files;
pub mod select_files;
pub mod select_options;

pub fn select_files() -> (HashSet<FileInfo>, FileConfig) {
    return select_files::main();
}

pub fn inspect_files(dir_files: &HashSet<FileInfo>) -> () {
    return inspect_files::main(dir_files);
}

pub fn select_options() -> Options {
    return select_options::main();
}

pub fn resize_files(dir_files: &HashSet<FileInfo>, config: &Config) -> () {
    return resize_files::main(dir_files, config);
}

pub fn rename_files(config: &Config) -> () {
    return rename_files::main(config);
}

pub fn optimize_files(dir_files: &HashSet<FileInfo>, config: &Config) -> () {
    return optimize_files::main(dir_files, config);
}

pub fn compare_files(dir_files: HashSet<FileInfo>, config: &Config) -> () {
    return compare_files::main(dir_files, &config);
}
