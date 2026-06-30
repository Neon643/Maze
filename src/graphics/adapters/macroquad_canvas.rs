use macroquad::prelude::*;

use crate::graphics::canvas::Canvas;
use crate::graphics::draw_color::DrawColor;
use crate::graphics::geometry::line_segment::LineSegment;
use crate::graphics::geometry::rect_area::RectArea;

pub struct MacroquadCanvas;

impl MacroquadCanvas {
    pub fn new() -> Self {
        Self
    }
    fn macroquad_color(color: DrawColor) -> Color {
        Color::new(color.r(), color.g(), color.b(), color.a())
    }
}

impl Canvas for MacroquadCanvas {
    fn clear(&mut self, color: DrawColor) {
        clear_background(MacroquadCanvas::macroquad_color(color));
    }

    fn fill_rect(&mut self, area: RectArea, color: DrawColor) {
        draw_rectangle(
            area.x(),
            area.y(),
            area.width(),
            area.height(),
            MacroquadCanvas::macroquad_color(color),
        );
    }

    fn stroke_line(&mut self, line: LineSegment, thickness: f32, color: DrawColor) {
        let from = line.from();
        let to = line.to();
        draw_line(
            from.x(),
            from.y(),
            to.x(),
            to.y(),
            thickness,
            MacroquadCanvas::macroquad_color(color),
        );
    }

    fn text(&mut self, text: &str, x: f32, y: f32, font_size: f32, color: DrawColor) {
        draw_text(
            text,
            x,
            y,
            font_size,
            MacroquadCanvas::macroquad_color(color),
        );
    }
}
