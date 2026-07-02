use crate::domain::{maze::Maze, position::Position};

pub struct SolvationTask {
    maze: Maze,
    start: Position,
    finish: Position,
}
/// Maze path solving task.
///
/// Contains maze and boundary positions for solvation algorithm.
impl SolvationTask {
    /// Creates task for finding path from start to finish.
    pub fn new(maze: Maze, start: Position, finish: Position) -> Self {
        Self {
            maze,
            start,
            finish,
        }
    }
    /// Checks whether start and finish positions belong to maze.
    ///
    /// This method does not check that path between positions exists.
    /// It only validates task boundaries.
    pub fn valid(&self) -> bool {
        self.maze.contains(self.start) && self.maze.contains(self.finish)
    }
}
