mod console;
mod domain;
mod generator;
mod graphics;
mod solver;

use crate::console::maze_printer::MazePrinter;
use crate::domain::maze::Maze;
use crate::domain::position::Position;
use crate::generator::generation_step::GenerationStep;
use crate::generator::wilson_generator::WilsonGenerator;
use crate::graphics::maze_painter::MazePainter;
use crate::solver::depth_first_solver::DepthFirstSolver;
use crate::solver::search_step::SearchStep;

use macroquad::experimental::camera::mouse;
use macroquad::miniquad::start;
use macroquad::prelude::*;

enum AppState {
    Idle,
    Generating,
    Finished,
    Solving,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Maze".to_string(),
        window_width: 1000,
        window_height: 760,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let width = 20;
    let height = 20;
    let start = Position::new(height - 1, width - 1);
    let finish = Position::new(0, 0);
    let mut generator = WilsonGenerator::new();
    let mut visible_maze = Maze::new(width, height);
    let mut step_index = 0;
    let mut state = AppState::Idle;
    let button_x = 680.0;
    let button_y = 150.0;
    let button_w = 220.0;
    let button_h = 40.0;

    let solve_button_x = 680.0;
    let solve_button_y = 250.0;
    let solve_button_w = 220.0;
    let solve_button_h = 40.0;

    let mut search_steps: Vec<SearchStep> = Vec::new();
    let mut search_step_index = 0;
    let mut current_path: Vec<Position> = Vec::new();

    loop {
        clear_background(DARKGRAY);

        let (mouse_x, mouse_y) = mouse_position();

        let mouse_inside = mouse_x >= button_x
            && mouse_x <= button_x + button_w
            && mouse_y >= button_y
            && mouse_y <= button_y + button_h;

        let mouse_inside_solve = mouse_x >= solve_button_x
            && mouse_x <= solve_button_x + solve_button_w
            && mouse_y >= solve_button_y
            && mouse_y <= solve_button_y + solve_button_h;

        if mouse_inside && is_mouse_button_pressed(MouseButton::Left) {
            generator = WilsonGenerator::new();
            let _final_maze = generator.generate(width, height);

            visible_maze = Maze::new(width, height);
            step_index = 0;
            current_path.clear();
            search_steps.clear();
            search_step_index = 0;
            state = AppState::Generating;
        }

        if matches!(state, AppState::Generating) {
            if step_index < generator.steps().len() {
                match &generator.steps()[step_index] {
                    GenerationStep::PassageOpened { from, to } => {
                        visible_maze.open_passage(*from, *to);
                    }
                    _ => {}
                }
                step_index += 1;
            } else {
                state = AppState::Finished;
            }
        }

        if matches!(state, AppState::Finished)
            && mouse_inside_solve
            && is_mouse_button_pressed(MouseButton::Left)
        {
            let (_path, steps) = DepthFirstSolver::solve_with_steps(&visible_maze, start, finish);

            search_steps = steps;
            search_step_index = 0;
            current_path.clear();
            state = AppState::Solving;
        }

        if matches!(state, AppState::Solving) {
            if search_step_index < search_steps.len() {
                match &search_steps[search_step_index] {
                    SearchStep::Entered(position) => {
                        current_path.push(*position);
                    }
                    SearchStep::Backtraced(_) => {
                        current_path.pop();
                    }
                    SearchStep::Finished(path) => {
                        current_path = path.clone();
                    }
                }
                search_step_index += 1;
            } else {
                state = AppState::Finished;
            }
        }

        draw_rectangle(button_x, button_y, button_w, button_h, LIGHTGRAY);
        draw_rectangle(
            solve_button_x,
            solve_button_y,
            solve_button_w,
            solve_button_h,
            LIGHTGRAY,
        );

        draw_text(
            "Generate maze",
            button_x + 16.0,
            button_y + 27.0,
            30.0,
            BLACK,
        );

        draw_text(
            "Solve maze",
            solve_button_x + 16.0,
            solve_button_y + 27.0,
            30.0,
            BLACK,
        );

        MazePainter::draw(&visible_maze, &current_path, start, finish);
        // visible_maze.open_passage(from, to);
        next_frame().await;
    }
}

// fn main() {
//     let maze = WilsonGenerator::generate(20, 20);
//     //let mut test_maze = Maze::new(3, 3);
//     let start = Position::new(maze.height() - 1, maze.width() - 1);
//     let finish = Position::new(0, 0);
//     // let test_pos3 = Position::new(1, 1);
//     // let test_pos4 = Position::new(0, 1);
//     // let test_pos5 = Position::new(0, 0);

//     // test_maze.open_passage(test_pos1, test_pos2);
//     // test_maze.open_passage(test_pos2, test_pos3);
//     // test_maze.open_passage(test_pos3, test_pos4);
//     // test_maze.open_passage(test_pos4, test_pos5);

//     let printer = MazePrinter::render(&maze);
//     println!("{}", &printer);

//     let found_path = DepthFirstSolver::solve(&maze, start, finish);
//     println!("{:?}", &found_path)
// }
