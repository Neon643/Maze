use crate::graphics::rect_area::RectArea;

pub trait Pointer {
    fn clicked_inside(&self, area: &RectArea) -> bool;
}
