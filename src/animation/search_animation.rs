use crate::domain::position::Position;
use crate::solvation::SearchEvent;

pub struct SearchAnimation {
    steps: Vec<SearchEvent>,
    index: usize,
}

impl SearchAnimation {
    pub fn new(steps: Vec<SearchEvent>) -> Self {
        Self { steps, index: 0 }
    }

    pub fn tick(&mut self, path: &mut Vec<Position>) {
        if self.index < self.steps.len() {
            match &self.steps[self.index] {
                SearchEvent::Visited(position) => {
                    path.push(*position);
                }
                SearchEvent::Discarded(position) => {
                    if path.last() == Some(position) {
                        path.pop();
                    }
                }
                SearchEvent::Finished(final_path) => {
                    *path = final_path.clone();
                }
            }
            self.index += 1;
        }
    }

    pub fn is_finished(&self) -> bool {
        self.index >= self.steps.len()
    }
}
