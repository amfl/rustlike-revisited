
/// A Point value is just a named struct for a location within the 2D space
/// that roguelikes take place within.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Point {
    /// The x coordinate.
    x: i32,

    /// The y coordinate.
    y: i32,
}

impl Point {
    /// Constructs a new Point with the x and y coordinates specified.
    pub fn new(nx: i32, ny: i32) -> Point {
        Point { x: nx, y: ny }
    }

    /// Produces the point that is offset from this point by the delta x and
    /// delta y values specified.
    ///
    /// ```rust
    /// // Point::new(1,1).offset(3,25) == Point::new(4,26)
    /// ```
    pub fn offset(self, dx: i32, dy: i32) -> Point {
        Point {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    /// The x coordinate of the Point.
    pub fn x(self) -> i32 {
        self.x
    }

    /// The y coordinate of the Point.
    pub fn y(self) -> i32 {
        self.y
    }
}
