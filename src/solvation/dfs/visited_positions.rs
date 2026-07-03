use crate::domain::position::Position;
use std::collections::HashSet;

/// Positions already reached by DFS.
///
/// `VisitedPositions` hides collection type and gives DFS only domain behavior:
/// mark position as visited and check whether position was reached before.
#[derive(Default)]
pub struct VisitedPositions {
    positions: HashSet<Position>,
}

impl VisitedPositions {
    /// Creates empty visited positions collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Marks position as visited.
    pub fn visit(&mut self, position: Position) {
        self.positions.insert(position);
    }

    /// Checks whether position was already reached.
    pub fn reached(&self, position: Position) -> bool {
        self.positions.contains(&position)
    }
}
