use crate::domain::maze::Maze;
use crate::domain::position::Position;

#[derive(Debug)]
pub struct MazePrinter;

impl MazePrinter {
    pub fn render(maze: &Maze) -> String {
        let mut result = String::new();
        let horizontal = String::from("+---");
        let vertical = String::from("|");
        let blank = String::from("   ");
        let blank_horizontal = String::from("+   ");
        let blank_vertical = String::from(" ");
        let horizontal_endline = String::from("+\n");
        let vertical_endline = String::from("\n");
        result.push_str(&horizontal.repeat(maze.width()));
        result.push_str(&horizontal_endline);
        for row in 0..maze.height() {
            let mut current_vertical = String::from(&vertical);
            let mut current_horizontal = String::new();
            for col in 0..maze.width() {
                let current_position = Position::new(row, col);
                let position_down = Position::new(row + 1, col);
                let position_right = Position::new(row, col + 1);
                current_vertical.push_str(&blank);
                if maze.has_passage(current_position, position_right) {
                    current_vertical.push_str(&blank_vertical);
                } else {
                    current_vertical.push_str(&vertical);
                }
                if maze.has_passage(current_position, position_down) {
                    current_horizontal.push_str(&blank_horizontal);
                } else {
                    current_horizontal.push_str(&horizontal);
                }
            }
            result.push_str(&current_vertical);
            result.push_str(&vertical_endline);
            result.push_str(&current_horizontal);
            result.push_str(&horizontal_endline);
        }
        result
    }
}
