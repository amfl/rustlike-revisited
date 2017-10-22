extern crate rlr;
extern crate pancurses;
#[macro_use]
extern crate log;
extern crate env_logger;

fn main() {
    // Initialize the logger in the main executable.
    // Libraries will simply include `log` and use the macros.
    // Can always just not set it up and there is very little overhead!
    env_logger::init().unwrap();
    info!("Starting RLR.");

    let win = pancurses::initscr();

    let mut running = true;

    let screen_width = 30;
    let screen_height = 15;

    let mut player_x = screen_width / 2;
    let mut player_y = screen_height / 2;

    while running {
        win.mvprintw(player_x, player_y, "@");
        win.refresh();

        let input = win.getch();
        win.mvprintw(player_x, player_y, " ");

        match input {
            Some(pancurses::Input::Character('q')) => { running = false; },
            Some(x) => {rlr::input_handlers::handle_keys(x); },
            None => {},
        }
    }

    pancurses::endwin();
}
