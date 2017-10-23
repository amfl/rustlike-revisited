pub struct Entity {
    pub x: i32,
    pub y: i32,
    pub glyph: char
}

impl Entity {
    pub fn mov(self: &mut Self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}
