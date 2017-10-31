extern crate pancurses;
extern crate specs;

use entity::Color;
use map::Map;
use component::{Position, BaseEntity};
use std::collections::HashMap;
use specs::{World, Join};

pub struct Renderer {
    pairs: HashMap<(Color, Color), u32>,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer { pairs: HashMap::new() }
    }

    /// Pancurses requires some intialization stuff be called once per program
    /// in order to start up properly.
    /// There is probably a better way to do this.
    pub fn static_init() {
        pancurses::start_color();
        // This is required for being able to use -1 as the default color.
        pancurses::use_default_colors();
    }

    /// Given a foreground and background color, we return the pancurses ID for the color pair.
    /// If none exists, we create one.
    fn get_pair(self: &mut Self, fg: Color, bg: Color) -> u32 {
        let tup = (fg, bg);

        // This would be nice, but we need to perform some pancurses actions if this color pair doesn't exist
        // self.pairs.entry(tup).or_insert(new_pair_id).clone()

        if self.pairs.contains_key(&tup) {
            self.pairs.get(&tup).unwrap().clone()
        }
        else {
            let new_pair_id = (self.pairs.len() + 1) as u32;
            pancurses::init_pair(new_pair_id as i16, tup.0.clone() as i16, tup.1.clone() as i16);
            info!("Inserted new color: {:?} {:?} as {}", tup.0, tup.1, new_pair_id);
            self.pairs.insert(tup, new_pair_id);
            new_pair_id as u32
        }
    }

    /// Render the map and all entities
    pub fn render_all(self: &mut Self, win: &pancurses::Window, world: &World) {
        let map = world.read_resource::<Map>();
        // Render the map
        for y in 0..map.data.len() {
            for x in 0..map.data[0].len() {
                let tile = map.data[y][x];
                if !tile.transparent && !tile.walkable {
                    win.mvprintw(y as i32, x as i32, "#");
                }
                else {
                    win.mvprintw(y as i32, x as i32, ".");
                }
            }
        }

        // Render all entities
        let positions = world.read::<Position>();
        let baseEnt = world.read::<BaseEntity>();
        for entity in world.entities().join() {
            if let Some(base) = baseEnt.get(entity) {
                if let Some(pos) = positions.get(entity) {
                    win.attrset(pancurses::COLOR_PAIR(self.get_pair(base.fg.clone(), base.bg.clone())));
                    win.mvprintw(pos.y, pos.x, &base.glyph.to_string());
                }
            }
        }
        win.attrset(pancurses::COLOR_PAIR(0));

        // for ent in entities.iter() {
        //     self.draw_entity(win, ent);
        // }
    }

    pub fn clear_all(self: &mut Self, win: &pancurses::Window, world: &World) {
        // for ent in entities.iter() {
        //     self.clear_entity(win, ent);
        // }
    }

    // pub fn draw_entity(self: &mut Self, win: &pancurses::Window, ent: &Entity) {
    //     win.attrset(pancurses::COLOR_PAIR(self.get_pair(ent.fg.clone(), ent.bg.clone())));
    //     win.mvprintw(ent.y, ent.x, &ent.glyph.to_string());
    // }

    // pub fn clear_entity(self: &mut Self, win: &pancurses::Window, ent: &Entity) {
    //     win.attrset(pancurses::COLOR_PAIR(0));
    //     win.mvprintw(ent.y, ent.x, " ");
    // }
}
