use std::collections::{HashMap, HashSet};

use crate::{
    display::splash::{hr, spacer, step},
    operations::{compare::compare_files, files::FileInfo},
    questions::compare::compare_results,
};

pub fn main(input_files: &HashSet<FileInfo>, config: &mut HashMap<&str, String>) -> () {
    step("Step 5: Compare results 📊");
    spacer();

    let start_compare = compare_results();
    if start_compare {
        compare_files(input_files, config.get("output_path").unwrap());
        hr();
        spacer();
        println!("Thanks for using this tool 🙏");
    }
}
