use crate::operations::{compare::FileReport, files::FileInfo};
use prettytable::{cell, row, Cell, Row, Table};
use size_format::SizeFormatterBinary;
use std::collections::HashSet;

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

fn format_diff(diff: i128) -> String {
    if diff < 0 {
        return "- ".to_owned()
            + SizeFormatterBinary::from::<u64>(
                u64::try_into(diff.abs().try_into().unwrap()).unwrap(),
            )
            .to_string()
            .as_str()
            + "B";
    } else {
        return "+ ".to_owned()
            + SizeFormatterBinary::from::<u64>(
                u64::try_into(diff.abs().try_into().unwrap()).unwrap(),
            )
            .to_string()
            .as_str()
            + "B";
    }
}

fn get_correct_cell(diff: &i128) -> Cell {
    if diff < &0 {
        return Cell::new(&format_diff(*diff)).style_spec("Fgb");
    }
    return Cell::new(&format_diff(*diff)).style_spec("Frb");
}

pub fn display_report_table(files: HashSet<FileReport>) {
    let mut table = Table::new();
    table.set_titles(row![
        "Name",
        "Resolution",
        "Original size",
        "Optimized size",
        "Difference",
    ]);

    for file in files {
        table.add_row(Row::new(vec![
            Cell::new(&file.name),
            Cell::new(&file.resolution),
            Cell::new(&file.original_formatted_size).style_spec("Fm"),
            Cell::new(&file.optimized_formatted_size).style_spec("Fmb"),
            get_correct_cell(&file.size_diff),
        ]));
    }

    table.printstd();
}
