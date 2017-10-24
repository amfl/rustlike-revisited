extern crate pancurses;
use event::{Event};

pub fn handle_keys(user_input: pancurses::Input) -> Option<Event> {
    info!("Pushed key: {:?}", user_input);
    match user_input {
        pancurses::Input::Character('h') => { info!("left"); Some(Event::Movement((-1, 0))) },
        pancurses::Input::Character('j') => { info!("down"); Some(Event::Movement((0, 1))) },
        pancurses::Input::Character('k') => { info!("up"); Some(Event::Movement((0, -1))) },
        pancurses::Input::Character('l') => { info!("right"); Some(Event::Movement((1, 0))) },
        pancurses::Input::Character('q') => { info!("Quitting!"); Some(Event::Quit) },
        _ => { None }
    }
}
