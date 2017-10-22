extern crate pancurses;

fn main() {
    println!("Wew!");

    // let locale_conf = pancurses::LcCategory::all;
    // // pancurses::setlocale(locale_conf, "en_US.UTF-8");
    // // pancurses::setlocale(locale_conf, "zh_CN.UTF-8");
    // pancurses::setlocale(locale_conf, "en_US.UTF-8");

    // /* Allow for extended keyboard (like F1). */
    // pancurses::keypad(pancurses::stdscr(), true);
    // pancurses::noecho();

    let win = pancurses::initscr();

    // Try and print out some stuff
    // pancurses::printw("Foobar\n");
    // UTF-8
    win.printw(".rs  R──────────────│││││││\n");
    win.printw("Great Firewall dislike VPN protocol.\nGFW 不喜欢 VPN 协议。");

    win.refresh();

    // Chuck a getch down so we don't immediately exit
    win.getch();

    pancurses::endwin();
}
