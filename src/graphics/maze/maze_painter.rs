use crate::domain::maze::Maze;
use crate::domain::position::Position;
use crate::graphics::canvas::Canvas;
use crate::graphics::geometry::line_segment::LineSegment;
use crate::graphics::geometry::rect_area::RectArea;
use crate::graphics::maze::maze_layout::MazeLayout;
use crate::graphics::maze::maze_painter_style::MazePainterStyle;

pub struct MazePainter {
    area: RectArea,
    style: MazePainterStyle,
}

impl MazePainter {
    pub fn new(area: RectArea, style: MazePainterStyle) -> Self {
        Self { area, style }
    }
    pub fn draw(
        &self,
        canvas: &mut dyn Canvas,
        maze: &Maze,
        path: &[Position],
        start: Position,
        finish: Position,
    ) {
        let layout = MazeLayout::new(self.area, maze.size());
        canvas.fill_rect(self.area, self.style.background_color());
        let start = layout.cell_area(start);
        let finish = layout.cell_area(finish);
        canvas.fill_rect(start, self.style.start_color());
        canvas.fill_rect(finish, self.style.finish_color());

        for pair in path.windows(2) {
            let from = pair[0];
            let to = pair[1];

            let from_center = layout.cell_area(from).center();
            let to_center = layout.cell_area(to).center();
            let line = LineSegment::new(from_center, to_center);
            canvas.stroke_line(line, layout.cell_size() / 2.0, self.style.path_color());
        }

        for row in 0..maze.height() {
            for col in 0..maze.width() {
                let current_position = Position::new(row, col);
                let position_down = Position::new(row + 1, col);
                let position_right = Position::new(row, col + 1);

                let cell = layout.cell_area(current_position);
                if row == 0 {
                    self.draw_wall(canvas, cell.top_edge());
                }
                if col == 0 {
                    self.draw_wall(canvas, cell.left_edge());
                }
                if !maze.has_passage(current_position, position_right) {
                    self.draw_wall(canvas, cell.right_edge());
                }
                if !maze.has_passage(current_position, position_down) {
                    self.draw_wall(canvas, cell.bottom_edge());
                }
            }
        }
    }
    fn draw_wall(&self, canvas: &mut dyn Canvas, wall: LineSegment) {
        canvas.stroke_line(wall, self.style.wall_thickness(), self.style.wall_color());
    }
}
