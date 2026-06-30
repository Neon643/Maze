use crate::graphics::geometry::line_segment::LineSegment;
use crate::graphics::geometry::point::Point;

#[derive(Clone, Copy)]
pub struct RectArea {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl RectArea {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> RectArea {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    pub fn contains(&self, point: Point) -> bool {
        point.x() >= self.x
            && point.x() <= self.x + self.width
            && point.y() <= self.y + self.height
            && point.y() >= self.y
    }
    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn width(&self) -> f32 {
        self.width
    }
    pub fn height(&self) -> f32 {
        self.height
    }
    pub fn center(&self) -> Point {
        Point::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }
    pub fn top_edge(&self) -> LineSegment {
        LineSegment::new(
            Point::new(self.x, self.y),
            Point::new(self.x + self.width, self.y),
        )
    }
    pub fn right_edge(&self) -> LineSegment {
        LineSegment::new(
            Point::new(self.x + self.width, self.y),
            Point::new(self.x + self.width, self.y + self.height),
        )
    }
    pub fn bottom_edge(&self) -> LineSegment {
        LineSegment::new(
            Point::new(self.x, self.y + self.height),
            Point::new(self.x + self.width, self.y + self.height),
        )
    }
    pub fn left_edge(&self) -> LineSegment {
        LineSegment::new(
            Point::new(self.x, self.y),
            Point::new(self.x, self.y + self.height),
        )
    }
}
