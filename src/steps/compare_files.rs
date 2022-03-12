use std::collections::HashSet;

use crate::{
    display::{
        splash::{spacer, step},
        tables::display_report_table,
    },
    operations::{
        compare::{compare_files, FileReport},
        files::FileInfo,
    },
    questions::compare::compare_results,
    Config,
};

pub fn main(input_files: HashSet<FileInfo>, config: &Config) -> () {
    step("Step 5: Compare results 📊");
    spacer();

    let start_compare = compare_results();
    if start_compare {
        let tables_set: HashSet<FileReport> =
            compare_files(input_files, config.output_path.to_string());

        // Display tables
        display_report_table(tables_set);
    }
}
