use termion::color;

pub fn do_splash() {
    spacer();
    println!("{}", color::Fg(color::Blue));
    println!(
        "{}
       MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
       MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMWWMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
       MMMMMMMMMMMMMMMMMMMMMMMMMWMMMXo;;cOWWMMMMMMMMMMMMMMMMMMMMMMMMMMM
       MMMMMMMMMMMMMMMMMMMMMMMMMWMNx'    .:OWMMMMMMMMMMMMMMMMMMMMMMMMMM
       MMMMMMMMMMMMMMMMMMMMMMMMMNk;        .lKMMMMMMMMMMMMMMMMMMMMMMMMM
       MMMMMMMMMMMMMMMMMMMMMMMMKc.           .xNMMMMMMMMMMMMMMMMMMMMMMM
       MMMMMMMMMMMMMMMMMMMMMWNk'              .:0WMWMMMMMMMMMMMMMMMMMMM
       MMMMMMMMMMMMMMMMMMMMWXl.   .';::::;.     .xNMMMMMMMMMMMMMMMMMMMM
       MMMMMMMMMMMMMMMMMMMWO,   ,d0NWWMMMWXkc.   .oXMMMMMMMMMMMMMMMMMMM
       MMMMMMMMMMMMMMMMMMWx.  .xNMMMMMMMMMMMW0;    :KWMMMMMMMMMMMMMMMMM
       MMMMMMMMMMMMMMMMMNd.  .xWWMMMMMMMMMMMMMXc    ,0WWMMMMMMMMMMMMMMM
       MMMMMMMMMMMMMMMMNo.   :NMMMMMMMMMMMMMMWM0'    'OWMMMMMMMMMMMMMMM
       MMMMMMMMMMMMMMMNl.    oMWMMMMMMMMMMMMWWWWo     'OWMMMMMMMMMMMMMM
       MMMMMMMMMMMMMMNl     .kNxlOXWMWMMMMNOo:xWk.     'OWMMMMMMMMMMMMM
       MMMMMMMMMMMMWWd.     '0Wk,.,l0WMMW0l,'cKMK,      ,0MMMMMMMMMMMMM
       MMMMMMMMMMMMWx.      ;XMMNK0KNWWMWWNXNWMMX:       ;KMMMMMMMMMMMM
       MMMMMMMMMMMMO'       cNMMMMMMMMMMMMMMMMMMWl        cNMMMMMMMMMMM
       MMMMMMMMMMMX;        oMMMMMMMMMMMMMMMMMMMMo        .dWMMMMMMMMMM
       MMMMMMMMMMWo         dMMMMMMMMMMMMMMMMMMMMx.        .OMMMMMMMMMM
       MMMMMMMMWWO'        .xMMMMMMMMMMMMMMMMMMMMx.         :XMMMMMMMMM
       MMMMMMMMMNc         .kMMMMMMMMMMMMMMMMMMMMO.         .dWMMMMMMMM
       MMMMMMMMMk.       ..;0MMMMMMMMMMMMMMMMMMMM0:...       ,0MMMMMMMM
       MMMMMMMMWd';codxO0KXNWMMMMMMMMMMMMMMMMMMMMWNXXK0Oxdl:,,xWMMMMMMM
       MMMMMMMMWNNWMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMWXXWMMMMMMM
       MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMWWMMMMMMMMM
       MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMWMMMMMMMMM
       MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM

",
        color::Fg(color::Blue)
    );
    println!("{}
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
", color::Fg(color::Magenta));
    spacer();
    println!(
        "{} cargo imager machine: version 0.1.0 author: @spaceout.pl",
        color::Fg(color::Reset)
    );
    hr();
}

pub fn hr() {
    println!("{}", color::Fg(color::LightMagenta));
    println!(
        "{}",
        "=============================================================================================================================================="
    );
    println!("{}", color::Fg(color::Reset));
}

pub fn spacer() {
    println!("{}", color::Fg(color::Reset));
    println!("{}", color::Fg(color::Reset));
}

pub fn step(string: &str) {
    println!("{}", color::Fg(color::LightGreen));
    println!("{}", string);
    println!("{}", color::Fg(color::Reset));
}
