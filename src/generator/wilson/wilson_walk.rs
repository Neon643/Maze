use crate::domain::maze::Maze;
use crate::domain::position::Position;
use crate::generator::wilson::generation_step::GenerationStep;

pub struct WilsonWalk {
    path: Vec<Position>,
}

impl WilsonWalk {
    pub fn new(start: Position) -> Self {
        Self { path: vec![start] }
    }

    pub fn current(&self) -> Position {
        self.path[self.path.len() - 1]
    }

    pub fn step_to(&mut self, next: Position) {
        if let Some(index) = self.path.iter().position(|position| *position == next) {
            self.path.truncate(index + 1);
        } else {
            self.path.push(next);
        }
    }

    pub fn positions(&self) -> &[Position] {
        &self.path
    }

    pub fn carve_into(&self, maze: &mut Maze, steps: &mut Vec<GenerationStep>) {
        for pair in self.path.windows(2) {
            let from = pair[0];
            let to = pair[1];
            maze.open_passage(from, to);
            steps.push(GenerationStep::PassageOpened { from, to });
        }
    }
}
