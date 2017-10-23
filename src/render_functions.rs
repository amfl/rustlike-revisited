extern crate pancurses;

use entity::Entity;

// fn render_all(win: &pancurses::Window, entities, screen_width, screen_height) {
//     // Draw all entities

// }

pub fn clear_entity(win: &pancurses::Window, ent: &Entity) {
    win.mvprintw(ent.y, ent.x, " ");
}

pub fn draw_entity(win: &pancurses::Window, ent: &Entity) {
    win.mvprintw(ent.y, ent.x, &ent.glyph.to_string());
}
