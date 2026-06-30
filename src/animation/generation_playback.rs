use crate::animation::generation_animation::GenerationAnimation;
use crate::domain::maze::Maze;

pub struct GenerationPlayback {
    animation: GenerationAnimation,
    maze: Maze,
}

impl GenerationPlayback {
    pub fn new(animation: GenerationAnimation, maze: Maze) -> Self {
        Self { animation, maze }
    }
    pub fn tick(&mut self) {
        self.animation.tick(&mut self.maze);
    }
    pub fn is_finished(&self) -> bool {
        self.animation.is_finished()
    }
    pub fn maze(&self) -> &Maze {
        &self.maze
    }
}
