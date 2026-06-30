use crate::domain::maze::Maze;
use crate::domain::maze_size::MazeSize;
use crate::domain::position::Position;

pub struct MazeSetup {
    size: MazeSize,
}

impl MazeSetup {
    pub fn new(size: MazeSize) -> Self {
        Self { size }
    }
    pub fn start(&self) -> Position {
        Position::new(self.size.height() - 1, self.size.width() - 1)
    }
    pub fn finish(&self) -> Position {
        Position::new(0, 0)
    }
    pub fn blank_maze(&self) -> Maze {
        Maze::new(self.size)
    }
}
