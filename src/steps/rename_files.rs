use crate::{
    display::{
        progress::progress_bar,
        splash::{hr, spacer, step},
    },
    operations::{
        files::{get_files_info, FileInfo},
        rename::rename_file,
    },
    questions::{new_name::get_new_name, rename::rename_files},
    Config,
};

pub fn main(config: &Config) -> () {
    step("Step 7: rename output files");
    spacer();

    let rename = rename_files();

    if rename {
        let mut new_name = get_new_name();

        let mut check_for_extension = new_name.contains(".");

        if check_for_extension {
            println!("No extension needed");
            new_name = get_new_name();
        }

        check_for_extension = new_name.contains(".");

        if check_for_extension {
            println!("You gonna mess up the files !!!");
            new_name = get_new_name();
        }

        let dir_files = get_files_info(config.output_path.as_str());

        let progress_bar = progress_bar(&dir_files);
        progress_bar.tick();

        let mut vec_files = dir_files.iter().collect::<Vec<&FileInfo>>();

        vec_files.sort();

        for (i, file) in vec_files.iter().enumerate() {
            let rfile = rename_file(
                file.path.to_string(),
                format!(
                    "{}/{}-{}.{}",
                    config.output_path,
                    new_name,
                    i + 1,
                    file.name.split('.').last().unwrap()
                ),
            );

            match rfile {
                Ok(_) => {
                    if i == dir_files.len() - 1 {
                        progress_bar.finish();
                    } else {
                        progress_bar.inc(1);
                    }
                }
                Err(error) => println!("{:#?}", error),
            }
        }

        spacer();
        println!("Success 🚀 : All files renamed");
        spacer();
        hr();
        println!();
        // Machine is going away soon
    }

    hr();
    spacer();
}
