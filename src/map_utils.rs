use map::Map;

pub struct Rect {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Rect {
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Rect{ x1: x, x2: x+w, y1: y, y2: y+h }
    }
}

pub fn make_room(map: &mut Map, room: &Rect) {
    for x in room.x1..room.x2 {
        for y in room.y1..room.y2 {
            let tile = &mut map.data[y][x];
            tile.walkable = true;
            tile.transparent = true;
        }
    }
}

pub fn make_map(map: &mut Map) {
    make_room(map, &Rect::new(2, 2, 10, 6));
    make_room(map, &Rect::new(10, 6, 6, 6));
    make_room(map, &Rect::new(20, 16, 3, 1));
}
