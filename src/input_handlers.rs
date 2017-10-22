extern crate pancurses;

pub fn handle_keys(user_input: pancurses::Input) {
    info!("Pushed key: {:?}", user_input);
    match user_input {
        pancurses::Input::Character('h') => { info!("left") },
        pancurses::Input::Character('j') => { info!("down") },
        pancurses::Input::Character('k') => { info!("up") },
        pancurses::Input::Character('l') => { info!("right") },
        _ => {}
    }
}
