use crate::graphics::button::Button;
use crate::graphics::button_style::ButtonStyle;
use crate::graphics::control_panel::ControlPanel;
use crate::graphics::draw_color::DrawColor;
use crate::graphics::maze_painter::MazePainter;
use crate::graphics::maze_painter_style::MazePainterStyle;
use crate::graphics::rect_area::RectArea;

pub struct AppLayout;

impl AppLayout {
    pub fn new() -> Self {
        Self
    }

    pub fn control_panel(&self) -> ControlPanel {
        ControlPanel::new(self.generate_button(), self.solve_button())
    }

    pub fn maze_painter(&self) -> MazePainter {
        MazePainter::new(self.maze_area(), self.maze_style())
    }

    fn generate_button(&self) -> Button {
        Button::new(
            "Generate maze",
            RectArea::new(680.0, 150.0, 220.0, 40.0),
            ButtonStyle::new(DrawColor::LIGHT_GRAY, DrawColor::BLACK, 30.0),
        )
    }

    fn solve_button(&self) -> Button {
        Button::new(
            "Solve maze",
            RectArea::new(680.0, 250.0, 220.0, 40.0),
            ButtonStyle::new(DrawColor::LIGHT_GRAY, DrawColor::BLACK, 30.0),
        )
    }

    fn maze_area(&self) -> RectArea {
        RectArea::new(40.0, 40.0, 600.0, 600.0)
    }

    fn maze_style(&self) -> MazePainterStyle {
        MazePainterStyle::new(
            2.0,
            DrawColor::BLACK,
            DrawColor::WHITE,
            DrawColor::GREEN,
            DrawColor::RED,
            DrawColor::RED,
        )
    }
}
