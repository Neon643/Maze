use crate::domain::maze::Maze;
use crate::generator::wilson::generation_step::GenerationStep;

pub struct WilsonGeneration {
    maze: Maze,
    steps: Vec<GenerationStep>,
}

impl WilsonGeneration {
    pub fn new(maze: Maze, steps: Vec<GenerationStep>) -> Self {
        Self { maze, steps }
    }
    pub fn into_parts(self) -> (Maze, Vec<GenerationStep>) {
        (self.maze, self.steps)
    }
}
