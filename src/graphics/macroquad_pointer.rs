use macroquad::prelude::*;

use crate::graphics::point::Point;
use crate::graphics::pointer::Pointer;
use crate::graphics::rect_area::RectArea;

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
