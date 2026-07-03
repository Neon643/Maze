use crate::domain::position::Position;

/// Current DFS path.
///
/// `SearchPath` owns path positions and hides raw vector operations from DFS.
#[derive(Default)]
pub struct SearchPath {
    positions: Vec<Position>,
}

impl SearchPath {
    /// Creates empty search path.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds position to current path.
    pub fn push(&mut self, position: Position) {
        self.positions.push(position);
    }

    /// Removes last position from current path.
    pub fn backtrack(&mut self) -> Option<Position> {
        self.positions.pop()
    }
}
