use crate::{
    display::{
        progress::progress_bar,
        splash::{hr, spacer, step},
    },
    operations::{
        files::{get_files_info, FileInfo},
        webp::convert_to_webp,
    },
    questions::webp::convert_files_to_webp,
    Config,
};

pub fn main(config: &Config) -> () {
    step("Step 8: convert files to webp");
    spacer();

    let convert = convert_files_to_webp();

    if convert {
        let dir_files = get_files_info(config.output_path.as_str());

        let progress_bar = progress_bar(&dir_files);
        progress_bar.tick();

        let mut vec_files = dir_files.iter().collect::<Vec<&FileInfo>>();

        vec_files.sort();

        for (i, file) in vec_files.iter().enumerate() {
            convert_to_webp(
                &file.path.to_string(),
                &format!(
                    "{}/{}.webp",
                    config.output_path,
                    file.name.split('.').into_iter().collect::<Vec<&str>>()[0]
                ),
            );

            if i == dir_files.len() - 1 {
                progress_bar.finish();
            } else {
                progress_bar.inc(1);
            }
        }

        spacer();
        println!("Success 🚀 : All files converted");
        spacer();
        // Machine is going away soon
    }

    hr();
    spacer();
    println!("Thanks for using this tool 🙏");
    spacer();
}
