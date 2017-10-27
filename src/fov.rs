use point::Point;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::i32;
use std::ops::Fn;

// /////////////////////////////////////////////////////////////////////////
// Public API
// /////////////////////////////////////////////////////////////////////////

/// If a type is a VisionCheck, it can determine if vision is blocked or not
/// at a given location. It must work for any potential location at all, so
/// out of bounds locations should simply return `true`.
pub trait VisionCheck {
    /// Should return if the point specified blocks vision.
    fn blocked(&self, Point) -> bool;
}

/// Obviously any function from `Point` to `bool` can be used as an instance
/// of `VisionCheck`.
impl<F: Fn(Point) -> bool> VisionCheck for F {
    fn blocked(&self, target: Point) -> bool {
        self(target)
    }
}

/// Uses the input set as the locations that do not block vision.
pub struct ClearSet(HashSet<Point>);

impl VisionCheck for ClearSet {
    fn blocked(&self, target: Point) -> bool {
        !self.0.contains(&target)
    }
}

/// Uses the input set as the locations that block vision.
pub struct BlockerSet(HashSet<Point>);

impl VisionCheck for BlockerSet {
    fn blocked(&self, target: Point) -> bool {
        self.0.contains(&target)
    }
}

/// Uses the input set as the locations that do not block vision (ref
/// version).
pub struct RefClearSet<'a>(&'a HashSet<Point>);

impl<'a> VisionCheck for RefClearSet<'a> {
    fn blocked(&self, target: Point) -> bool {
        !self.0.contains(&target)
    }
}

/// Uses the input set as the locations that block vision (ref version).
pub struct RefBlockerSet<'a>(&'a HashSet<Point>);

impl<'a> VisionCheck for RefBlockerSet<'a> {
    fn blocked(&self, target: Point) -> bool {
        self.0.contains(&target)
    }
}

/// Given a vision checker, range of vision, and starting location, determines
/// all locations that can be seen using the Precise Permissive Field of View
/// technique. Worst case runtime is O(n^2). The `range` is a `u32` because
/// `range` must be non-negative, and that's the easiest way to signal such a
/// requirement. The actual maximum computed range is 1 less than the `i32`
/// maximum. It seems more useful to ensure that you don't accidentally pass
/// in a negative range value compared to the "problem" of your results being
/// cut off past the 2 billion tiles away mark. For that matter, if your start
/// location and your range put together cause an integer overflow, well
/// that's bad for you too I guess.
///
/// http://www.roguebasin.com/index.php?title=Precise_Permissive_Field_of_View
pub fn ppfov<F: VisionCheck>(vision: &F, range: u32, start: Point) -> HashSet<Point> {
    let mut visited = HashSet::new();
    visited.insert(start);
    // We get the range value into a good place here (equal to or greater than
    // zero, and at least 1 less than the `i32` maximum) and from here on we
    // can just use all `i32` values, because it's a whole lot easier to do
    // work when everything is the same signed-ness.
    let irange: i32 = min(i32::MAX - 1, range as i32);
    check_quadrant(vision, irange, start, &mut visited, Quadrant::One);
    check_quadrant(vision, irange, start, &mut visited, Quadrant::Two);
    check_quadrant(vision, irange, start, &mut visited, Quadrant::Three);
    check_quadrant(vision, irange, start, &mut visited, Quadrant::Four);
    visited
}

// /////////////////////////////////////////////////////////////////////////
// Private API
// /////////////////////////////////////////////////////////////////////////

/// Checks an entire quadrant by sweeping along -1 sloped lines that move
/// progressively outward while maintaining a list of "views" that are
/// currently active. The same pattern is performed for each quadrant, and the
/// quadrant value itself determines when to flip x and/or y as needed.
fn check_quadrant<F: VisionCheck>(vision: &F,
                                  range: i32,
                                  start: Point,
                                  visited: &mut HashSet<Point>,
                                  quadrant: Quadrant) {
    let mut active_views = Vec::new();
    let shallow_line = SightLine::new(0, 1, range as i32, 0);
    let steep_line = SightLine::new(1, 0, 0, range as i32);
    active_views.push(View::new(shallow_line, steep_line));
    let max_index = 2 * range + 1;
    // This loop produces a pattern of moving outwards through the quadrant
    // along diagonals with a slope of -1 (that is, shaped kinda like a '\').
    for i in 1..max_index {
        let start_j = max(i - range, 0);
        let max_j = min(i, range) + 1;
        for j in start_j..max_j {
            visit_coord(vision,
                        i - j,
                        j,
                        start,
                        visited,
                        quadrant,
                        &mut active_views);
            if active_views.is_empty() {
                return;
            }
        }
    }
}

/// Attempts to visits a particular coordinate and updates the view as
/// necessary. It's possible that the specified location exists within no
/// views, in which case it will be bypassed.
fn visit_coord<F: VisionCheck>(vision: &F,
                               dx: i32,
                               dy: i32,
                               start: Point,
                               visited: &mut HashSet<Point>,
                               quadrant: Quadrant,
                               active_views: &mut Vec<View>) {
    let top_left = Point::new(dx, dy + 1);
    let bottom_right = Point::new(dx + 1, dy);
    let mut view_index_tmp = 0;
    // TODO: I bet we could maybe make this loop faster with optimizations on
    // by using an iterator. It seems a small thing, but at least this much of
    // the function executes for every single square in a quadrant as long as
    // there is even a single view left in that quadrant, so savings here
    // might get us a good win. Probably best to not fiddle with it until we
    // have a way to gather benchmark data though.
    while view_index_tmp < active_views.len() &&
          active_views[view_index_tmp].steep_line.is_below_or_collinear(bottom_right) {
        view_index_tmp += 1;
    }
    let view_index = view_index_tmp;
    if view_index == active_views.len() ||
       active_views[view_index].shallow_line.is_above_or_collinear(top_left) {
        // we didn't find a correct view, so we return early without adding
        // this location.
        return;
    } else {
        let true_location = start.offset(dx * quadrant.sign_x(), dy * quadrant.sign_y());
        visited.insert(true_location);
        if vision.blocked(true_location) {
            let shallow_above_bottom_right =
                active_views[view_index].shallow_line.is_above(bottom_right);
            let steep_below_top_left =
                active_views[view_index].steep_line.is_below(top_left);
            match (shallow_above_bottom_right, steep_below_top_left) {
                (true, true) => {
                    active_views.remove(view_index);
                }
                (true, false) => {
                    add_shallow_and_check(active_views, view_index, top_left)
                }
                (false, true) => {
                    add_steep_and_check(active_views, view_index, bottom_right)
                }
                (false, false) => {
                    let target = active_views[view_index].clone();
                    active_views.insert(view_index, target);
                    // these two lines must happen in this specific order in
                    // case the higher-indexed view fails the checking.
                    add_shallow_and_check(active_views, view_index + 1, top_left);
                    add_steep_and_check(active_views, view_index, bottom_right);
                }
            }
        }
    }
}

/// Adds then checks.
fn add_shallow_and_check(active_views: &mut Vec<View>,
                         view_index: usize,
                         shallow_bump: Point) {
    add_shallow_bump(&mut active_views[view_index], shallow_bump);
    check_view(active_views, view_index);
}

/// Adds then checks.
fn add_steep_and_check(active_views: &mut Vec<View>,
                       view_index: usize,
                       steep_bump: Point) {
    add_steep_bump(&mut active_views[view_index], steep_bump);
    check_view(active_views, view_index);
}

/// Adds a shallow bump to the view specified.
fn add_shallow_bump(view: &mut View, loc: Point) {
    view.shallow_line.x_final = loc.x();
    view.shallow_line.y_final = loc.y();
    // We extract the old view bump, then "cons" the new view bump onto the
    // old view bump. This could be a single line but since it's a little
    // goofy we'll leave it as two for a while.
    let old_bump = view.shallow_bump.take();
    view.shallow_bump = Some(ViewBump::new(loc, old_bump));
    let mut maybe_cur_bump = &view.steep_bump;
    while let &Some(ref cur_bump) = maybe_cur_bump {
        if view.shallow_line.is_above(cur_bump.location) {
            view.shallow_line.x_initial = cur_bump.location.x();
            view.shallow_line.y_initial = cur_bump.location.y();
        }
        maybe_cur_bump = cur_bump.parent.as_ref();
    }
}

/// adds a steep bump to the view specified.
fn add_steep_bump(view: &mut View, loc: Point) {
    view.steep_line.x_final = loc.x();
    view.steep_line.y_final = loc.y();
    // We extract the old view bump, then "cons" the new view bump onto the
    // old view bump. This could be a single line but since it's a little
    // goofy we'll leave it as two for a while.
    let old_bump = view.steep_bump.take();
    view.steep_bump = Some(ViewBump::new(loc, old_bump));
    let mut maybe_cur_bump = &view.shallow_bump;
    while let &Some(ref cur_bump) = maybe_cur_bump {
        if view.steep_line.is_below(cur_bump.location) {
            view.steep_line.x_initial = cur_bump.location.x();
            view.steep_line.y_initial = cur_bump.location.y();
        }
        maybe_cur_bump = cur_bump.parent.as_ref();
    }
}

/// Checks that the specified view is still valid, and removes it from the
/// vector if it is not. In this case, a view is invalid if the shallow and
/// steep line are collinear with each other and they extrude from the bottom
/// right or upper left corner of the origin.
fn check_view(active_views: &mut Vec<View>, view_index: usize) {
    let shallow_line = active_views[view_index].shallow_line;
    let steep_line = active_views[view_index].steep_line;
    if shallow_line.is_line_collinear(steep_line) &&
       (shallow_line.is_collinear(Point::new(0, 1)) ||
        shallow_line.is_collinear(Point::new(1, 0))) {
        active_views.remove(view_index);
    }
}

/// One of the four cartesian quadrants.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Quadrant {
    One,
    Two,
    Three,
    Four,
}

impl Quadrant {
    fn sign_x(self) -> i32 {
        match self {
            Quadrant::One | Quadrant::Four => 1,
            Quadrant::Two | Quadrant::Three => -1,
        }
    }
    fn sign_y(self) -> i32 {
        match self {
            Quadrant::One | Quadrant::Two => 1,
            Quadrant::Three | Quadrant::Four => -1,
        }
    }
}

/// One of the two lines of a View.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct SightLine {
    x_initial: i32,
    y_initial: i32,
    x_final: i32,
    y_final: i32,
}

impl SightLine {
    fn new(nxi: i32, nyi: i32, nxf: i32, nyf: i32) -> SightLine {
        SightLine {
            x_initial: nxi,
            y_initial: nyi,
            x_final: nxf,
            y_final: nyf,
        }
    }

    fn is_above(self, p: Point) -> bool {
        self.relative_slope(p) < 0
    }

    fn is_above_or_collinear(self, p: Point) -> bool {
        self.relative_slope(p) <= 0
    }

    fn is_below(self, p: Point) -> bool {
        self.relative_slope(p) > 0
    }

    fn is_below_or_collinear(self, p: Point) -> bool {
        self.relative_slope(p) >= 0
    }

    fn is_collinear(self, p: Point) -> bool {
        self.relative_slope(p) == 0
    }

    fn is_line_collinear(self, other: SightLine) -> bool {
        self.is_collinear(Point::new(other.x_initial, other.y_initial)) &&
        self.is_collinear(Point::new(other.x_final, other.y_final))
    }

    fn relative_slope(self, p: Point) -> i32 {
        ((self.y_final - self.y_initial) * (self.x_final - p.x())) -
        ((self.x_final - self.x_initial) * (self.y_final - p.y()))
    }
}

/// One of the locations that has bumped into a View during the View's
/// lifetime. Also links to it's "parent", linked list style, so that you can
/// scan all the way back.
#[derive(Debug, PartialEq, Eq, Clone)]
struct ViewBump {
    location: Point,
    parent: Box<Option<ViewBump>>,
}

impl ViewBump {
    fn new(nloc: Point, nparent: Option<ViewBump>) -> ViewBump {
        ViewBump {
            location: nloc,
            parent: Box::new(nparent),
        }
    }
}

/// A cone of vision going out from the start location.
#[derive(Debug, PartialEq, Eq, Clone)]
struct View {
    shallow_bump: Option<ViewBump>,
    shallow_line: SightLine,
    steep_bump: Option<ViewBump>,
    steep_line: SightLine,
}

impl View {
    fn new(n_shallow: SightLine, n_steep: SightLine) -> View {
        View {
            shallow_line: n_shallow,
            steep_line: n_steep,
            shallow_bump: None,
            steep_bump: None,
        }
    }
}

// /////////////////////////////////////////////////////////////////////////
// Testing Code
// /////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use point::*;
    use std::collections::HashSet;
    use std::ops::Range;

    /// The start location should always be in the output, even if you give a
    /// range of 0.
    quickcheck!{
        fn ppfov_start_always_in_output(x: i32, y: i32) -> bool {
            let start = Point::new(x,y);
            let output = ppfov(&(|p| true ), 0, start);
            output.contains(&start)
        }
    }

    /// You should always be able to see though a diagonal gap.
    quickcheck!{
        fn diagonal_gaps_allow_sight(x: i32, y: i32) -> bool {
            let start = Point::new(x,y);
            let output = ppfov(&(|p: Point| p.x() == x || p.y() == y ), 10, start);
            output.contains(&Point::new(x+1,y+1)) &&
                output.contains(&Point::new(x+1,y-1)) &&
                output.contains(&Point::new(x-1,y+1)) &&
                output.contains(&Point::new(x-1,y-1))
        }
    }

    /// Perfectly square rooms should always produce the correct number of
    /// seen rooms.
    quickcheck!{
        fn square_room_count_correct() -> bool {
            let start = Point::new(0,0);

            let intended_values: Vec<usize> = (0..7)
                .map(|z| (2*(z+1)+1)*(2*(z+1)+1))
                .collect();

            let actual_results: Vec<usize> = (0..7)
                .map(|z| ppfov(&(|p: Point| p.x().abs() > z || p.y().abs() > z),
                                (z+1) as u32,
                                start))
                .map(|hs| hs.len())
                .collect();

            intended_values == actual_results
        }
    }

    /// Given a pillar at (x,y), this computes the cells around that location
    /// paired up with the opposite side of the pillar.
    fn pillar_pairs(x: i32, y: i32) -> [(Point, Point); 8] {
        [(Point::new(x - 1, y - 1), Point::new(x + 1, y + 1)),
         (Point::new(x - 1, y), Point::new(x + 1, y)),
         (Point::new(x - 1, y + 1), Point::new(x + 1, y - 1)),
         (Point::new(x, y - 1), Point::new(x, y + 1)),
         // skip when both are plu-mi zero
         (Point::new(x, y + 1), Point::new(x, y - 1)),
         (Point::new(x + 1, y - 1), Point::new(x - 1, y + 1)),
         (Point::new(x + 1, y), Point::new(x - 1, y)),
         (Point::new(x + 1, y + 1), Point::new(x - 1, y - 1))]
    }

    /// You should never be able to see to the far side of a 1x1 pillar.
    quickcheck!{
        fn can_never_see_around_pillar(x: i32, y: i32) -> bool {
            let pillar_vision = |p: Point| p.x() == x && p.y() == y;
            pillar_pairs(x,y).into_iter().all(|&(start,inverse)|
                !ppfov(&pillar_vision,3,start).contains(&inverse))
        }
    }

    /// I swear that this function was saner in Haskell!
    fn kuo_clear(start: Point, length: i32) -> Vec<(ClearSet, Point)> {
        let mut output = Vec::new();
        // this indents less with a list monad, lemme tell ya.
        for dx in -1..2 {
            for dy in -1..2 {
                for is_vert in [false, true].into_iter() {
                    // Starting data
                    let x = start.x();
                    let y = start.y();
                    let end = if *is_vert {
                        Point::new(x + (dx * 2), y + (dy * 2) + (length + 1) * dy)
                    } else {
                        Point::new(x + (dx * 2) + (length + 1) * dx, y + (dy * 2))
                    };
                    // Build up the initial hallway
                    let mut spaces = HashSet::new();
                    spaces.insert(start);
                    if *is_vert {
                        spaces.insert(Point::new(x, y + dy))
                    } else {
                        spaces.insert(Point::new(x + dx, y))
                    };
                    if *is_vert {
                        spaces.insert(Point::new(x, y + (dy * 2)))
                    } else {
                        spaces.insert(Point::new(x + (dx * 2), y))
                    };

                    // hallway loop
                    for len in 0..length {
                        if *is_vert {
                            spaces.insert(Point::new(x + dx, y + (dy * 2) + (len * dy)));
                        } else {
                            spaces.insert(Point::new(x + (dx * 2) + (len * dx), y + dy));
                        }
                    }

                    // finish off the hallway
                    if *is_vert {
                        spaces.insert(Point::new(x + (dx * 2),
                                                 y + (dy * 2) + (length - 1) * dy))
                    } else {
                        spaces.insert(Point::new(x + (dx * 2) + (length - 1) * dx,
                                                 y + (dy * 2)))
                    };
                    if *is_vert {
                        spaces.insert(Point::new(x + (dx * 2),
                                                 y + (dy * 2) + (length) * dy))
                    } else {
                        spaces.insert(Point::new(x + (dx * 2) + (length) * dx,
                                                 y + (dy * 2)))
                    };

                    spaces.insert(end);
                    output.push((ClearSet(spaces), end));
                }
            }
        }
        output
    }

    /// You should always be able to see to the end of a kuo corridor.
    quickcheck!{
        fn can_always_see_kuo_corridor(s: u64, i: u64) -> bool {
            use pcgen::PCGen;
            let mut gen = PCGen::new(s,i);
            const NEG_MIL: i32 = -1_000_000;
            const POS_MIL: i32 = 1_000_000;
            let locx = gen.in_range(NEG_MIL,POS_MIL);
            let locy = gen.in_range(NEG_MIL,POS_MIL);
            let cor_length = gen.in_range(2,100);
            let start = Point::new(locx,locy);
            kuo_clear(start,cor_length).into_iter().all(|(vision,end)|
                ppfov(&vision, (cor_length+11) as u32, start).contains(&end))
        }
    }
}
