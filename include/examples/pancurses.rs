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
    win.printw("Great Firewall dislike VPN protocol.\nGFW 不喜欢 VPN 协议。\n");

    //////////////////////////////////////////////////////////
    // Try adding some colors
    if !pancurses::has_colors() {
        win.printw("Your terminal doesn't support colors.\n");
    } else {
        pancurses::start_color();

        // This is required for being able to use -1 as the default color.
        pancurses::use_default_colors();

        // Pairs start at 1. 0 is the default.
        pancurses::init_pair(1, pancurses::COLOR_RED, -1);
        win.printw("test string goes here\n");
        win.attron(pancurses::COLOR_PAIR(1));
        win.printw("test string goes here\n");
        win.attroff(pancurses::COLOR_PAIR(1));
        win.printw("test string goes here\n");

        // Can enable attributes one at a time...
        win.attron(pancurses::A_BOLD);
        win.printw("test string goes here\n");

        // Or brute-force set them in one go with OR
        win.attrset(pancurses::COLOR_PAIR(1) |pancurses::A_BOLD);
        win.printw("test string goes here\n");
        win.attroff(pancurses::A_BOLD);
        win.printw("test string goes here\n");
    }

    //////////////////////////////////////////////////////////
    // Call refresh so we can see our changes on screen
    win.refresh();

    // Chuck a getch down so we don't immediately exit
    win.getch();

    pancurses::endwin();
}
