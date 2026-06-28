use crate::domain::direction::Direction;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Cell {
    opened_walls: HashSet<Direction>,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            opened_walls: HashSet::new(),
        }
    }

    pub fn open(&mut self, direction: Direction) {
        self.opened_walls.insert(direction);
    }

    pub fn is_open(&self, direction: Direction) -> bool {
        self.opened_walls.contains(&direction)
    }
}
