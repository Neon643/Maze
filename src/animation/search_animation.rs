use crate::domain::position::Position;
use crate::solver::search_step::SearchStep;

pub struct SearchAnimation {
    steps: Vec<SearchStep>,
    index: usize,
}

impl SearchAnimation {
    pub fn new(steps: Vec<SearchStep>) -> Self {
        Self { steps, index: 0 }
    }

    pub fn tick(&mut self, path: &mut Vec<Position>) {
        if self.index < self.steps.len() {
            match &self.steps[self.index] {
                SearchStep::Entered(position) => {
                    path.push(*position);
                }
                SearchStep::Backtracked => {
                    path.pop();
                }
                SearchStep::Finished(final_path) => {
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
