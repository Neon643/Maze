use crate::graphics::geometry::point::Point;

#[derive(Clone, Copy)]
pub struct LineSegment {
    from: Point,
    to: Point,
}
impl LineSegment {
    pub fn new(from: Point, to: Point) -> Self {
        Self { from, to }
    }
    pub fn from(&self) -> Point {
        self.from
    }

    pub fn to(&self) -> Point {
        self.to
    }
}
