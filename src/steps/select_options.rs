use std::collections::HashMap;

use crate::{
    display::splash::{spacer, step},
    questions::options::get_options,
};

pub fn main(config: &mut HashMap<&str, String>) -> () {
    // Ask for output questions;
    step("Step 3: Select options 🏷️");

    let options = get_options();
    config.insert("options", options.to_string());

    spacer();
}
