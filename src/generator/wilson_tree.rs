use crate::domain::position::Position;
use crate::generator::wilson_walk::WilsonWalk;
use std::collections::HashSet;

pub struct WilsonTree {
    positions: HashSet<Position>,
}

impl WilsonTree {
    pub fn new(root: Position) -> Self {
        let positions: HashSet<Position> = vec![root].into_iter().collect();
        Self { positions }
    }

    pub fn contains(&self, position: Position) -> bool {
        self.positions.contains(&position)
    }

    pub fn len(&self) -> usize {
        self.positions.len()
    }

    pub fn missing_from(&self, positions: &[Position]) -> Vec<Position> {
        let candidates: Vec<Position> = positions
            .iter()
            .copied()
            .filter(|position| !self.contains(*position))
            .collect();
        candidates
    }

    fn include(&mut self, position: Position) {
        self.positions.insert(position);
    }

    pub fn include_walk(&mut self, walk: &WilsonWalk) {
        for position in walk.positions() {
            self.include(*position);
        }
    }
}
