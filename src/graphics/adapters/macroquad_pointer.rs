use macroquad::prelude::*;

use crate::graphics::geometry::point::Point;
use crate::graphics::geometry::rect_area::RectArea;
use crate::graphics::pointer::Pointer;

pub struct MacroquadPointer;

impl MacroquadPointer {
    pub fn new() -> Self {
        Self
    }
}

impl Pointer for MacroquadPointer {
    fn clicked_inside(&self, area: &RectArea) -> bool {
        let (x, y) = mouse_position();
        area.contains(Point::new(x, y)) && is_mouse_button_pressed(MouseButton::Left)
    }
}
