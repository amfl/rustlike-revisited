use map::Map;

pub fn make_map(map: &mut Map) {
    for row in map.data.iter_mut() {
        for tile in row.iter_mut() {
            tile.transparent = true;
            tile.walkable = true;
        }
    }
    for x in 6..9 {
        map.data[8][x].walkable = false;
        map.data[8][x].transparent = false;
    }
}
