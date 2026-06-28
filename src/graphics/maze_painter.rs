#[derive(Debug)]
pub struct MazePainter;

use std::fmt::Alignment::Center;

use crate::domain::maze::Maze;
use crate::domain::position::Position;
use macroquad::prelude::*;

impl MazePainter {
    pub fn draw(maze: &Maze, path: &[Position], start: Position, finish: Position) {
        draw_rectangle(40.0, 40.0, 600.0, 600.0, WHITE);
        let area_x = 40.0;
        let area_y = 40.0;
        let area_w = 600.0;
        let area_h = 600.0;
        let thickness = 2.0;

        let cell_w = area_w / maze.width() as f32;
        let cell_h = area_h / maze.height() as f32;

        let start_x = area_x + start.col() as f32 * cell_w;
        let start_y = area_y + start.row() as f32 * cell_h;

        let finish_x = area_x + finish.col() as f32 * cell_w;
        let finish_y = area_y + finish.row() as f32 * cell_h;

        draw_rectangle(start_x, start_y, cell_w, cell_h, RED);
        draw_rectangle(finish_x, finish_y, cell_w, cell_h, RED);

        for pair in path.windows(2) {
            let from = pair[0];
            let to = pair[1];

            let from_x = area_x + from.col() as f32 * cell_w + cell_w / 2.0;
            let from_y = area_y + from.row() as f32 * cell_h + cell_h / 2.0;

            let to_x = area_x + to.col() as f32 * cell_w + cell_w / 2.0;
            let to_y = area_y + to.row() as f32 * cell_h + cell_h / 2.0;

            draw_line(from_x, from_y, to_x, to_y, 14.0, GREEN);
        }

        for row in 0..maze.height() {
            for col in 0..maze.width() {
                let x = area_x + col as f32 * cell_w;
                let y = area_y + row as f32 * cell_h;
                // top:    (x, y) -> (x + cell_w, y)
                let top_x1 = x;
                let top_y1 = y;
                let top_x2 = x + cell_w;
                let top_y2 = y;
                // right:  (x + cell_w, y) -> (x + cell_w, y + cell_h)
                let right_x1 = x + cell_w;
                let right_y1 = y;
                let right_x2 = x + cell_w;
                let right_y2 = y + cell_h;
                // bottom: (x, y + cell_h) -> (x + cell_w, y + cell_h)
                let bottom_x1 = x;
                let bottom_y1 = y + cell_h;
                let bottom_x2 = x + cell_w;
                let bottom_y2 = y + cell_h;
                // left:   (x, y) -> (x, y + cell_h)
                let left_x1 = x;
                let left_y1 = y;
                let left_x2 = x;
                let left_y2 = y + cell_h;

                let current_position = Position::new(row, col);
                let position_down = Position::new(row + 1, col);
                let position_right = Position::new(row, col + 1);

                if row == 0 {
                    draw_line(top_x1, top_y1, top_x2, top_y2, thickness, BLACK);
                }

                if col == 0 {
                    draw_line(left_x1, left_y1, left_x2, left_y2, thickness, BLACK);
                }

                if !maze.has_passage(current_position, position_right) {
                    draw_line(right_x1, right_y1, right_x2, right_y2, thickness, BLACK);
                }
                if !maze.has_passage(current_position, position_down) {
                    draw_line(bottom_x1, bottom_y1, bottom_x2, bottom_y2, thickness, BLACK);
                }
            }
        }
        //     let mut result = String::new();
        //     let horizontal = String::from("+---");
        //     let vertical = String::from("|");
        //     let blank = String::from("   ");
        //     let blank_horizontal = String::from("+   ");
        //     let blank_vertical = String::from(" ");
        //     let horizontal_endline = String::from("+\n");
        //     let vertical_endline = String::from("\n");
        //     result.push_str(&horizontal.repeat(maze.width()));
        //     result.push_str(&horizontal_endline);
        //     for row in 0..maze.height() {
        //         let mut current_vertical = String::from(&vertical);
        //         let mut current_horizontal = String::new();
        //         for col in 0..maze.width() {
        //             let current_position = Position::new(row, col);
        //             let position_down = Position::new(row + 1, col);
        //             let position_right = Position::new(row, col + 1);
        //             current_vertical.push_str(&blank);
        //             if maze.has_passage(current_position, position_right) {
        //                 current_vertical.push_str(&blank_vertical);
        //             } else {
        //                 current_vertical.push_str(&vertical);
        //             }
        //             if maze.has_passage(current_position, position_down) {
        //                 current_horizontal.push_str(&blank_horizontal);
        //             } else {
        //                 current_horizontal.push_str(&horizontal);
        //             }
        //         }
        //         result.push_str(&current_vertical);
        //         result.push_str(&vertical_endline);
        //         result.push_str(&current_horizontal);
        //         result.push_str(&horizontal_endline);
        //     }
        //     result
        // }
    }
}
