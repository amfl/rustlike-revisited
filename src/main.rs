extern crate pancurses;

fn main() {
    println!("Hello, world!");

    let win = pancurses::initscr();

    let mut running = true;

    while running {
        let input = win.getch();
        match input {
            Some(pancurses::Input::Character('q')) => { running = false; },
            Some(_) => {},
            None => {},
        }
    }
}
