mod animation;
mod app;
mod domain;
mod generator;
mod graphics;
mod solvation;
mod solver;

use crate::app::maze_app::MazeApp;
use crate::app::maze_setup::MazeSetup;
use crate::domain::maze_size::MazeSize;
use crate::generator::random_positions::RandomPositions;
use crate::generator::wilson::wilson_generator::WilsonGenerator;
use crate::graphics::adapters::macroquad_canvas::MacroquadCanvas;
use crate::graphics::adapters::macroquad_pointer::MacroquadPointer;
use crate::graphics::app_layout::AppLayout;

use macroquad::prelude::*;

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
    let size = MazeSize::new(25, 25);
    let mut canvas = MacroquadCanvas::new();
    let pointer = MacroquadPointer::new();
    let layout = AppLayout::new();
    let mut app = MazeApp::new(
        MazeSetup::new(size),
        WilsonGenerator::new(size, RandomPositions::new()),
        layout.control_panel(),
        layout.maze_painter(),
    );
    loop {
        app.tick(&mut canvas, &pointer);
        next_frame().await;
    }
}
