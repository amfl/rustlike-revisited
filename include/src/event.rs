extern crate pancurses;

pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
pub enum Event {
    Movement((i32, i32)),
    Quit,
}

pub struct EventQueue(pub Vec<Event>);
