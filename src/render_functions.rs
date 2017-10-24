extern crate pancurses;

use entity::Entity;

pub fn render_all(win: &pancurses::Window, entities: &Vec<Entity>, screen_width: u32, screen_height: u32) {
    for ent in entities.iter() {
        draw_entity(win, ent);
    }
}

pub fn clear_all(win: &pancurses::Window, entities: &Vec<Entity>, screen_width: u32, screen_height: u32) {
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
