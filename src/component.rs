use specs::{Component, VecStorage, NullStorage};
use entity::Color;

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct MoveDelta {
    pub dx: i32,
    pub dy: i32,
}
impl Component for MoveDelta {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct BaseEntity {
    pub glyph: char,
    pub fg: Color,
    pub bg: Color,
    pub blocks: bool,
    pub name: String,
}
impl Component for BaseEntity {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct Puppeted;
impl Component for Puppeted {
    type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct Blocking;
impl Component for Blocking {
    type Storage = NullStorage<Self>;
}
