use std::cmp::Ordering;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        // a nonsense ordering but it keeps things consistent
        self.y.cmp(&other.y).then_with(|| self.x.cmp(&other.x))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
