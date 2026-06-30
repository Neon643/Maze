use crate::domain::maze::Maze;
use crate::generator::generation_step::GenerationStep;

pub struct GenerationAnimation {
    steps: Vec<GenerationStep>,
    index: usize,
}

impl GenerationAnimation {
    pub fn new(steps: Vec<GenerationStep>) -> Self {
        Self { steps, index: 0 }
    }

    pub fn tick(&mut self, maze: &mut Maze) {
        if self.index < self.steps.len() {
            match &self.steps[self.index] {
                GenerationStep::PassageOpened { from, to } => {
                    maze.open_passage(*from, *to);
                }
            }
            self.index += 1;
        }
    }

    pub fn is_finished(&self) -> bool {
        self.index >= self.steps.len()
    }

    pub fn empty() -> Self {
        Self::new(Vec::new())
    }
}
