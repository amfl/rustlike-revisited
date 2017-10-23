extern crate rlr;
extern crate pancurses;
#[macro_use]
extern crate log;
extern crate env_logger;

use rlr::event::{Event, Direction};
use rlr::entity::Entity;

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

    let mut player = Entity{
        x: screen_width / 2,
        y: screen_height / 2,
        glyph: '@'
    };

    while running {
        win.mvprintw(player.y, player.x, "@");
        win.refresh();
        pancurses::noecho();
        pancurses::curs_set(0);

        let input = win.getch();
        win.mvprintw(player.y, player.x, " ");

        match input {
            Some(x) => {
                match rlr::input_handlers::handle_keys(x) {
                    Some(event) => match event {
                        Event::Quit => { running = false; },
                        Event::Movement(direction) => {
                            match direction {
                                Direction::Left => { player.mov(-1, 0); }
                                Direction::Right => { player.mov(1, 0); }
                                Direction::Up => { player.mov(0, -1); }
                                Direction::Down => { player.mov(0, 1); }
                            };
                        },
                    },
                    None => {}
                }
            }
            None => {}
        }

    }

    pancurses::endwin();
}
