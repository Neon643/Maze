use crate::graphics::draw_color::DrawColor;
use crate::graphics::geometry::line_segment::LineSegment;
use crate::graphics::geometry::rect_area::RectArea;

pub trait Canvas {
    fn clear(&mut self, color: DrawColor);
    fn fill_rect(&mut self, area: RectArea, color: DrawColor);
    fn stroke_line(&mut self, line: LineSegment, thickness: f32, color: DrawColor);
    fn text(&mut self, text: &str, x: f32, y: f32, font_size: f32, color: DrawColor);
}
