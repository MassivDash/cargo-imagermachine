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
            Cell::new(&file.mime_type.to_string()),
            Cell::new(&file.resolution),
        ]));
    }

    table.printstd();
}

fn format_diff(diff: i128, original_size: &u64) -> String {
    let percentage = ((diff.abs() as f64 / *original_size as f64) * 100.0).round();

    if diff < 0 {
        return format!(
            "{}% (-{}B)",
            percentage,
            SizeFormatterBinary::from::<u64>(
                u64::try_into(diff.abs().try_into().unwrap()).unwrap(),
            ),
        );
    } else {
        return format!(
            "{}% (+{}B)",
            percentage,
            SizeFormatterBinary::from::<u64>(
                u64::try_into(diff.abs().try_into().unwrap()).unwrap(),
            ),
        );
    }
}

fn get_correct_cell(diff: &i128, original_size: &u64) -> Cell {
    if diff < &0 {
        return Cell::new(&format_diff(*diff, original_size)).style_spec("Fgb");
    }
    return Cell::new(&format_diff(*diff, original_size)).style_spec("Frb");
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
            get_correct_cell(&file.size_diff, &file.original_size),
        ]));
    }

    table.printstd();
}
