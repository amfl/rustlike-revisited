extern crate pancurses;

use entity::Entity;
use map::Map;

pub fn render_all(win: &pancurses::Window, map: &Map, entities: &Vec<Entity>) {
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

    for ent in entities.iter() {
        draw_entity(win, ent);
    }
}

pub fn clear_all(win: &pancurses::Window, entities: &Vec<Entity>) {
    for ent in entities.iter() {
        clear_entity(win, ent);
    }
}

pub fn draw_entity(win: &pancurses::Window, ent: &Entity) {
    win.mvprintw(ent.y, ent.x, &ent.glyph.to_string());
}

pub fn clear_entity(win: &pancurses::Window, ent: &Entity) {
    win.mvprintw(ent.y, ent.x, " ");
}
