use crate::animation::search_animation::SearchAnimation;
use crate::domain::position::Position;

pub struct SearchPlayback {
    animation: SearchAnimation,
    path: Vec<Position>,
}

impl SearchPlayback {
    pub fn new(animation: SearchAnimation) -> Self {
        Self {
            animation,
            path: Vec::new(),
        }
    }

    pub fn tick(&mut self) {
        self.animation.tick(&mut self.path);
    }

    pub fn is_finished(&self) -> bool {
        self.animation.is_finished()
    }

    pub fn path(&self) -> &[Position] {
        &self.path
    }
}
