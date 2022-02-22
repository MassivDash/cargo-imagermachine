use inquire::{error::InquireResult, Confirm, MultiSelect, Select, Text};
fn main() {
    println!("
d8b                                                                         888     d8b                 
Y8P                                                                         888     Y8P                 
                                                                            888                         
88888888b.d88b.  8888b.  .d88b.  .d88b. 888d88888888b.d88b.  8888b.  .d8888b88888b. 88888888b.  .d88b.  
888888  888  88b     88bd88P88bd8P  Y8b888P  888  888  88b     88bd88P   888   88b888888   88bd8P  Y8b 
888888  888  888.d888888888  88888888888888    888  888  888.d888888888     888  888888888  88888888888 
888888  888  888888  888Y88b 888Y8b.    888    888  888  888888  888Y88b.   888  888888888  888Y8b.     
888888  888  888 Y888888  Y88888  Y8888 888    888  888  888 Y888888  Y8888P888  888888888  888  Y8888  
                             888                                                                        
                        Y8b d88P                                                                        
                          Y88P     
");

    let input_path = Text::new("Path")
        .with_default(".")
        .with_help_message("Enter directory path (type . for current)")
        .prompt();

    match input_path {
        Ok(path) => println!("Using path {}", path),
        Err(_) => println!("An error happened when asking for your name, try again later."),
    }

    let output_path = Text::new("Output Path")
        .with_default(".")
        .with_help_message("Enter output directory path, default /out")
        .prompt();

    match output_path {
        Ok(path) => println!("Using path {}", path),
        Err(_) => println!("An error happened when asking for your name, try again later."),
    }

    let options = vec![
        "default optimization",
        "resize and optimize",
        "custom optimization",
        "add watermark",
    ];

    let ans = Select::new("What's your favorite fruit?", options).prompt();

    match ans {
        Ok(choice) => println!("{}! That's mine too!", choice),
        Err(_) => println!("There was an error, please try again"),
    }
}
