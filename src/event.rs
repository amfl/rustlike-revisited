extern crate pancurses;

pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub enum Event {
    Movement((i32, i32)),
    Quit,
}

pub struct IOEvent {
    pub input: Option<pancurses::Input>,
}
