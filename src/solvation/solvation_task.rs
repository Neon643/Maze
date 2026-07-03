use crate::domain::{direction::Direction, maze::Maze, position::Position};
/// Maze path solving task.
///
/// Contains maze and boundary positions for solvation algorithm.
pub struct SolvationTask {
    maze: Maze,
    start: Position,
    finish: Position,
}

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
    /// Returns maze positions directly reachable from given position.
    pub fn passable_neighbors(&self, position: Position) -> Vec<Position> {
        Direction::ALL
            .iter()
            .copied()
            .filter_map(|direction| self.maze.neighbor(position, direction))
            .filter(|neighbor| self.maze.has_passage(position, *neighbor))
            .collect()
    }
}
