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

    let screen_width: u32 = 30;
    let screen_height: u32 = 15;

    let mut player = Entity{
        x: (screen_width / 2) as i32,
        y: (screen_height / 2) as i32,
        glyph: '@'
    };
    let mut npc = Entity{
        x: 3,
        y: 3,
        glyph: '$'
    };
    let mut entities = vec![player, npc];

    while running {
        rlr::render_functions::render_all(&win, &entities, screen_width, screen_height);
        win.refresh();
        pancurses::noecho();
        pancurses::curs_set(0);

        let input = win.getch();
        rlr::render_functions::clear_entity(&win, &entities[0]);

        match input {
            Some(x) => {
                match rlr::input_handlers::handle_keys(x) {
                    Some(event) => match event {
                        Event::Quit => { running = false; },
                        Event::Movement(direction) => {
                            match direction {
                                Direction::Left => { entities[0].mov(-1, 0); }
                                Direction::Right => { entities[0].mov(1, 0); }
                                Direction::Up => { entities[0].mov(0, -1); }
                                Direction::Down => { entities[0].mov(0, 1); }
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
