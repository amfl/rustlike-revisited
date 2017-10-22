extern crate pancurses;
use event::Direction;

pub fn handle_keys(user_input: pancurses::Input) -> Option<Direction> {
    info!("Pushed key: {:?}", user_input);
    match user_input {
        pancurses::Input::Character('h') => { info!("left"); Some(Direction::Left) },
        pancurses::Input::Character('j') => { info!("down"); Some(Direction::Down) },
        pancurses::Input::Character('k') => { info!("up"); Some(Direction::Up) },
        pancurses::Input::Character('l') => { info!("right"); Some(Direction::Right) },
        _ => { None }
    }
}
