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

    /// Returns new path with position added.
    pub fn pushed(self, position: Position) -> Self {
        let mut positions = self.positions;
        positions.push(position);

        Self { positions }
    }

    /// Returns new path without last position and discarded position.
    pub fn backtracked(self) -> (Self, Option<Position>) {
        let mut positions = self.positions;
        let discarded = positions.pop();

        (Self { positions }, discarded)
    }
    /// Consumes path and returns stored positions.
    pub(super) fn into_vec(self) -> Vec<Position> {
        self.positions
    }
}
