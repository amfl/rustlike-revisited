extern crate rand;

// use rand::Rng;
use map_utils::rand::Rng;
// I have absolutely no idea why it thinks rand is in map_utils...

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

    /// Returns true if this rectangle intersects with another one
    pub fn intersects(self: &Self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 &&
            self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn center(self: &Self) -> (usize, usize) {
        ((self.x1 + self.x2) / 2,
         (self.y1 + self.y2) / 2)
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

/// Returns player starting position
pub fn make_map(map: &mut Map) -> (usize, usize) {
    let room_max_size = 10;
    let room_min_size = 6;
    let max_rooms = 30;

    let map_height = map.data.len();
    let map_width = map.data[0].len();

    let mut rooms: Vec<Rect> = Vec::new();
    for _ in 0..max_rooms {
        let mut rng = rand::thread_rng();
        let w = rng.gen_range::<usize>(room_min_size, room_max_size);
        let h = rng.gen_range::<usize>(room_min_size, room_max_size);
        let x = rng.gen_range::<usize>(0, map_width - w - 1);
        let y = rng.gen_range::<usize>(0, map_height - h - 1);

        let room = Rect::new(x, y, w, h);
        if rooms.iter().all(|it| !it.intersects(&room) ) {
            make_room(map, &room);
            rooms.push(room);
        }
    }

    rooms[0].center()

}
