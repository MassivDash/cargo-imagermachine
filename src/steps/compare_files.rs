use std::collections::{HashMap, HashSet};

use crate::{
    display::{
        splash::{hr, spacer, step},
        tables::display_report_table,
    },
    operations::{
        compare::{compare_files, FileReport},
        files::FileInfo,
    },
    questions::compare::compare_results,
};

pub fn main(input_files: HashSet<FileInfo>, config: &mut HashMap<&str, String>) -> () {
    step("Step 5: Compare results 📊");
    spacer();

    let start_compare = compare_results();
    if start_compare {
        let tables_set: HashSet<FileReport> =
            compare_files(input_files, config.get("output_path").unwrap().to_string());

        // Display tables
        display_report_table(tables_set);

        // Machine is going away soon
        hr();
        spacer();
        println!("Thanks for using this tool 🙏");
        spacer();
    }
}
