#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Tile {
    pub transparent: bool,
    pub walkable: bool,
}
impl Tile {
    pub fn new() -> Self {
        Tile{ transparent: false, walkable: false }
    }
}

pub struct Map {
    pub data: Vec<Vec<Tile>>
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        // TODO investigate whether I should switch to a dynamically sized array instead of a vec
        // https://stackoverflow.com/questions/13212212/creating-two-dimensional-arrays-in-rust

        let base = vec![Tile::new(); width];
        let grid = vec![base.clone(); height];

        Map{data: grid}
    }
}

#[cfg(test)]
mod tests {
    use map::Map;

    #[test]
    fn tiles_not_equal() {
        let mut map = Map::new(16, 16);
        map.data[2][3].transparent = false;
        map.data[3][2].transparent = true;
        assert_ne!(map.data[2][3], map.data[3][2]);
    }
}
