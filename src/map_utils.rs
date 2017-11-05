extern crate rand;
extern crate std;
extern crate specs;

// use rand::Rng;
use map_utils::rand::Rng;
// I have absolutely no idea why it thinks rand is in map_utils...

use std::cmp;
use specs::World;

use map::Map;
use entity::{Color};
use component::{BaseEntity, Position, MoveDelta, Blocking, Fighter};

pub struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: usize, h: usize) -> Self {
        Rect{ x1: x, x2: x+w as i32, y1: y, y2: y+h as i32 }
    }

    /// Returns true if this rectangle intersects with another one
    pub fn intersects(self: &Self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 &&
            self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn center(self: &Self) -> (i32, i32) {
        ((self.x1 + self.x2) / 2,
         (self.y1 + self.y2) / 2)
    }
}

pub fn place_entities(room: &Rect, world: &mut World, max_monsters_per_room: usize) {
    let mut rng = rand::thread_rng();
    let num_monsters = rng.gen_range::<usize>(0, max_monsters_per_room);

    // TODO I'm sure this is an anti-pattern. Use collect().uniq() or something.
    let mut mobs = Vec::new();
    for _ in 0..num_monsters {
        // Choose a random location in this room
        let x = rng.gen_range::<i32>(room.x1, room.x2);
        let y = rng.gen_range::<i32>(room.y1, room.y2);

        if !mobs.contains(&(x, y)) {
            mobs.push((x, y));
        }
    }

    for &(x, y) in mobs.iter() {
        world.create_entity()
            .with(Position { x: x, y: y })
            .with(Blocking)
            .with(MoveDelta { dx: 0, dy: 0 })
            .with(BaseEntity {
                    fg: Color::Green,
                    bg: Color::Default,
                    glyph: 'o',
                    blocks: true,
                    name: String::from("Orc"),
                })
            .with(Fighter {
                    max_hp: 10,
                    hp: 10,
                    power: 4,
                    def: 1,
                })
            .build();
    }
}

pub fn make_h_tunnel(map: &mut Map, x1: i32, x2: i32, y: i32) {
    for x in cmp::min(x1, x2)..cmp::max(x1, x2)+1 {
        let tile = map.at_mut(x, y);
        tile.walkable = true;
        tile.transparent = true;
    }
}

pub fn make_v_tunnel(map: &mut Map, y1: i32, y2: i32, x: i32) {
    for y in cmp::min(y1, y2)..cmp::max(y1, y2)+1 {
        let tile = &mut map.at_mut(x, y);
        tile.walkable = true;
        tile.transparent = true;
    }
}

pub fn make_room(map: &mut Map, room: &Rect) {
    for x in room.x1..room.x2 {
        for y in room.y1..room.y2 {
            let tile = &mut map.at_mut(x, y);
            tile.walkable = true;
            tile.transparent = true;
        }
    }
}

/// Returns player starting position
pub fn make_map(map: &mut Map, world: &mut World) -> (i32, i32) {
    let room_max_size = 10;
    let room_min_size = 6;
    let max_rooms = 30;
    let max_monsters_per_room = 4;

    let map_height = map.data.len();
    let map_width = map.data[0].len();

    let mut rooms: Vec<Rect> = Vec::new();
    for _ in 0..max_rooms {
        let mut rng = rand::thread_rng();
        let w = rng.gen_range::<usize>(room_min_size, room_max_size);
        let h = rng.gen_range::<usize>(room_min_size, room_max_size);
        let x = rng.gen_range::<i32>(0, (map_width - w) as i32 - 1);
        let y = rng.gen_range::<i32>(0, (map_height - h) as i32 - 1);

        let room = Rect::new(x, y, w, h);
        if rooms.iter().all(|it| !it.intersects(&room) ) {
            make_room(map, &room);

            // Join this room with the previous one
            if let Some(old_room) = rooms.last() {
                let (new_x, new_y) = room.center();
                let (old_x, old_y) = old_room.center();
                if rng.gen() {
                    // Horizontal first
                    info!("Horizontal first");
                    make_h_tunnel(map, old_x, new_x, old_y);
                    make_v_tunnel(map, old_y, new_y, new_x);
                }
                else {
                    // Vertical first
                    info!("Vertical first");
                    make_v_tunnel(map, old_y, new_y, old_x);
                    make_h_tunnel(map, old_x, new_x, new_y);
                }
            }

            place_entities(&room, world, max_monsters_per_room);

            rooms.push(room);
        }
    }

    rooms[0].center()

}
