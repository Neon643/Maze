use crate::graphics::canvas::Canvas;
use crate::graphics::geometry::rect_area::RectArea;
use crate::graphics::pointer::Pointer;
use crate::graphics::widgets::button_style::ButtonStyle;

pub struct Button {
    label: String,
    area: RectArea,
    style: ButtonStyle,
}

impl Button {
    pub fn new(label: &str, area: RectArea, style: ButtonStyle) -> Button {
        Self {
            label: label.to_string(),
            area,
            style,
        }
    }
    pub fn draw(&self, canvas: &mut dyn Canvas) {
        canvas.fill_rect(self.area, self.style.background());

        canvas.text(
            &self.label,
            self.area.x() + 16.0,
            self.area.y() + 27.0,
            self.style.font_size(),
            self.style.text(),
        );
    }
    pub fn is_clicked(&self, pointer: &dyn Pointer) -> bool {
        pointer.clicked_inside(&self.area)
    }
}
