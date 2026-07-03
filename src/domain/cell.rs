use crate::domain::direction::Direction;
use std::collections::HashSet;

#[derive(Debug, Clone, Default)]
pub struct Cell {
    opened_walls: HashSet<Direction>,
}

impl Cell {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(&mut self, direction: Direction) {
        self.opened_walls.insert(direction);
    }

    pub fn is_open(&self, direction: Direction) -> bool {
        self.opened_walls.contains(&direction)
    }
}
