use prettytable::{cell, row, Cell, Row, Table};
use std::collections::HashSet;

use crate::operations::files::FileInfo;

pub fn display_files_table(files: &HashSet<FileInfo>) {
    let mut table = Table::new();
    table.set_titles(row![
        "Path",
        "Name",
        "Size",
        "Size bytes",
        "Type",
        "Resolution"
    ]);

    for file in files {
        table.add_row(Row::new(vec![
            Cell::new(&file.path),
            Cell::new(&file.name).style_spec("Fmb"),
            Cell::new(&file.size_formatted).style_spec("Fmb"),
            Cell::new(&file.size_bytes.to_string()),
            Cell::new(&file.mime_type),
            Cell::new(&file.resolution),
        ]));
    }

    table.printstd();
}
