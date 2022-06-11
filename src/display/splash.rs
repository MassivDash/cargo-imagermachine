use termion::color;

pub fn do_splash() {
    spacer();
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
        color::Fg(color::Magenta)
    );
    println!(
        "{}
        ██╗███╗   ███╗ █████╗  ██████╗ ███████╗██████╗     ███╗   ███╗ █████╗  ██████╗██╗  ██╗██╗███╗   ██╗███████╗
        ██║████╗ ████║██╔══██╗██╔════╝ ██╔════╝██╔══██╗    ████╗ ████║██╔══██╗██╔════╝██║  ██║██║████╗  ██║██╔════╝
        ██║██╔████╔██║███████║██║  ███╗█████╗  ██████╔╝    ██╔████╔██║███████║██║     ███████║██║██╔██╗ ██║█████╗  
        ██║██║╚██╔╝██║██╔══██║██║   ██║██╔══╝  ██╔══██╗    ██║╚██╔╝██║██╔══██║██║     ██╔══██║██║██║╚██╗██║██╔══╝  
        ██║██║ ╚═╝ ██║██║  ██║╚██████╔╝███████╗██║  ██║    ██║ ╚═╝ ██║██║  ██║╚██████╗██║  ██║██║██║ ╚████║███████╗
        ╚═╝╚═╝     ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚═╝  ╚═╝    ╚═╝     ╚═╝╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝╚═╝╚═╝  ╚═══╝╚══════╝
                                                                                                                   ",
        color::Fg(color::Magenta)
    );
    spacer();
    println!(
        "{} cargo imager machine: version 0.1.2 author: @spaceout.pl",
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
