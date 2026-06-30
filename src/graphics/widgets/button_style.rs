use crate::graphics::draw_color::DrawColor;

pub struct ButtonStyle {
    background_color: DrawColor,
    text_color: DrawColor,
    font_size: f32,
}

impl ButtonStyle {
    pub fn new(background_color: DrawColor, text_color: DrawColor, font_size: f32) -> ButtonStyle {
        Self {
            background_color,
            text_color,
            font_size,
        }
    }
    pub fn background(&self) -> DrawColor {
        self.background_color
    }
    pub fn text(&self) -> DrawColor {
        self.text_color
    }
    pub fn font_size(&self) -> f32 {
        self.font_size
    }
}
