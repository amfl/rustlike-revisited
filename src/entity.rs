#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum Color {
    Default = -1,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

pub struct Entity {
    pub x: i32,
    pub y: i32,
    pub glyph: char,
    pub fg: Color,
    pub bg: Color,
    pub blocks: bool,
    pub name: String,
}

impl Entity {
    pub fn mov(self: &mut Self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

pub fn get_blocking_entities_at(entities: &Vec<Entity>, x: i32, y: i32) -> Vec<&Entity> {
    entities.iter().filter(|ent| ent.x == x && ent.y == y).collect()
}
