use crate::domain::position::Position;
use std::collections::HashSet;

/// Positions already reached by DFS.
///
/// `VisitedPositions` hides collection type and gives DFS only domain behavior:
/// create new visited collection with position included and check whether
/// position was reached before.
#[derive(Default)]
pub struct VisitedPositions {
    positions: HashSet<Position>,
}

impl VisitedPositions {
    /// Creates empty visited positions collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns new collection with position marked as visited.
    pub fn visited(self, position: Position) -> Self {
        let mut positions = self.positions;
        positions.insert(position);

        Self { positions }
    }

    /// Checks whether position was already reached.
    pub fn reached(&self, position: Position) -> bool {
        self.positions.contains(&position)
    }
}
