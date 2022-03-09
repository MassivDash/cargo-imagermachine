use std::collections::HashMap;

use crate::{
    display::splash::{spacer, step},
    questions::options::get_options,
};

pub enum Options {
    Default,
    Resize,
    Custom,
}

pub fn main() -> (Options) {
    // Ask for output questions;
    step("Step 3: Select options 🏷️");

    // "default optimization" | "resize and optimize" | "custom job"
    let options = get_options();

    spacer();

    return match options.as_str() {
        "default optimization" => Options::Default,
        "resize and optimize" => Options::Resize,
        "custom job" => Options::Custom,
        _ => panic!("Unknown option"),
    };
}
