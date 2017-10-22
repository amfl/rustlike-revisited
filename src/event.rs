pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub enum Event {
    Movement(Direction),
    Quit,
}
