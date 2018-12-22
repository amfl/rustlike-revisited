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

// pub fn get_blocking_entities_at(entities: &Vec<Entity>, x: i32, y: i32) -> Vec<&Entity> {
//     entities.iter().filter(|ent| ent.x == x && ent.y == y).collect()
// }
