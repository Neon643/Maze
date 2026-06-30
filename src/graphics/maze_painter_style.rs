use crate::graphics::draw_color::DrawColor;

pub struct MazePainterStyle {
    wall_thickness: f32,
    wall_color: DrawColor,
    background_color: DrawColor,
    path_color: DrawColor,
    start_color: DrawColor,
    finish_color: DrawColor,
}

impl MazePainterStyle {
    pub fn new(
        wall_thickness: f32,
        wall_color: DrawColor,
        background_color: DrawColor,
        path_color: DrawColor,
        start_color: DrawColor,
        finish_color: DrawColor,
    ) -> Self {
        Self {
            wall_thickness,
            wall_color,
            background_color,
            path_color,
            start_color,
            finish_color,
        }
    }

    pub fn wall_thickness(&self) -> f32 {
        self.wall_thickness
    }

    pub fn wall_color(&self) -> DrawColor {
        self.wall_color
    }

    pub fn background_color(&self) -> DrawColor {
        self.background_color
    }

    pub fn path_color(&self) -> DrawColor {
        self.path_color
    }

    pub fn start_color(&self) -> DrawColor {
        self.start_color
    }

    pub fn finish_color(&self) -> DrawColor {
        self.finish_color
    }
}
