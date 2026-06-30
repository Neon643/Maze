use crate::domain::maze_size::MazeSize;
use crate::domain::position::Position;
use crate::graphics::rect_area::RectArea;

pub struct MazeLayout {
    area: RectArea,
    size: MazeSize,
}

impl MazeLayout {
    pub fn new(area: RectArea, size: MazeSize) -> Self {
        Self { area, size }
    }
    pub fn cell_area(&self, position: Position) -> RectArea {
        let cell_w = self.area.width() / self.size.width() as f32;
        let cell_h = self.area.height() / self.size.height() as f32;
        let cell_x = self.area.x() + position.col() as f32 * cell_w;
        let cell_y = self.area.y() + position.row() as f32 * cell_h;

        RectArea::new(cell_x, cell_y, cell_w, cell_h)
    }
    pub fn cell_size(&self) -> f32 {
        self.cell_width().min(self.cell_height())
    }
    fn cell_width(&self) -> f32 {
        self.area.width() / self.size.width() as f32
    }
    fn cell_height(&self) -> f32 {
        self.area.height() / self.size.height() as f32
    }
}
