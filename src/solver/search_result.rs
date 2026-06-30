use crate::domain::position::Position;
use crate::solver::search_step::SearchStep;

pub struct SearchResult {
    path: Option<Vec<Position>>,
    steps: Vec<SearchStep>,
}

impl SearchResult {
    pub fn new(path: Option<Vec<Position>>, steps: Vec<SearchStep>) -> Self {
        Self { path, steps }
    }

    pub fn into_parts(self) -> (Option<Vec<Position>>, Vec<SearchStep>) {
        (self.path, self.steps)
    }
}
