use mazeAlgorithm::app::maze_app::MazeApp;
use mazeAlgorithm::app::maze_setup::MazeSetup;
use mazeAlgorithm::domain::maze_size::MazeSize;
use mazeAlgorithm::generator::random_positions::RandomPositions;
use mazeAlgorithm::generator::wilson::wilson_generator::WilsonGenerator;
use mazeAlgorithm::graphics::adapters::macroquad_canvas::MacroquadCanvas;
use mazeAlgorithm::graphics::adapters::macroquad_pointer::MacroquadPointer;
use mazeAlgorithm::graphics::app_layout::AppLayout;

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
