extern crate pancurses;
use event::{Event, Direction};

pub fn handle_keys(user_input: pancurses::Input) -> Option<Event> {
    info!("Pushed key: {:?}", user_input);
    match user_input {
        pancurses::Input::Character('h') => { info!("left"); Some(Event::Movement(Direction::Left)) },
        pancurses::Input::Character('j') => { info!("down"); Some(Event::Movement(Direction::Down)) },
        pancurses::Input::Character('k') => { info!("up"); Some(Event::Movement(Direction::Up)) },
        pancurses::Input::Character('l') => { info!("right"); Some(Event::Movement(Direction::Right)) },
        pancurses::Input::Character('q') => { info!("Quitting!"); Some(Event::Quit) },
        _ => { None }
    }
}
