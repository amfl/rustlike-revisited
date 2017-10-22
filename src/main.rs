extern crate pancurses;

fn main() {
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
            Some(_) => {},
            None => {},
        }
    }

    pancurses::endwin();
}
