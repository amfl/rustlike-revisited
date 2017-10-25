extern crate rlr;
extern crate pancurses;
#[macro_use]
extern crate log;
extern crate env_logger;

use rlr::event::Event;
use rlr::entity::Entity;
use rlr::map::Map;

fn main() {
    // Initialize the logger in the main executable.
    // Libraries will simply include `log` and use the macros.
    // Can always just not set it up and there is very little overhead!
    env_logger::init().unwrap();
    info!("Starting RLR.");

    let win = pancurses::initscr();

    let mut running = true;

    let mut map = Map::new(32, 32);
    rlr::map_utils::make_map(&mut map);

    let player = Entity{
        x: 8,
        y: 4,
        glyph: '@'
    };
    let npc = Entity{
        x: 3,
        y: 3,
        glyph: '$'
    };
    let mut entities = vec![player, npc];

    while running {
        rlr::render_functions::render_all(&win, &map, &entities);
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
                        Event::Movement((dx, dy)) => {
                            let player = &mut entities[0];
                            let pos = (player.x + dx, player.y + dy);
                            if map.data[pos.1 as usize][pos.0 as usize].walkable {
                                player.mov(dx, dy);
                            }
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
