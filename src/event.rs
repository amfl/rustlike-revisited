extern crate pancurses;
extern crate specs;

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

pub struct InputQueue(pub Vec<pancurses::Input>);
pub struct MovementIntent(pub Vec<(specs::Entity, (i32, i32))>);
// pub struct EventQueue(pub Vec<Event>);
