use crate::domain::position::Position;
use crate::solvation::dfs::{SearchEvents, SearchPath, VisitedPositions};

/// DFS trace composed from visited positions, current path and search events.
///
/// `SearchTrace` coordinates trace objects without exposing their internal
/// collections.
#[derive(Default)]
pub struct SearchTrace {
    visited: VisitedPositions,
    path: SearchPath,
    events: SearchEvents,
}

impl SearchTrace {
    /// Creates empty DFS trace.
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks whether position was already reached.
    pub fn reached(&self, position: Position) -> bool {
        self.visited.reached(position)
    }

    /// Records DFS entrance into position.
    pub fn enter(&mut self, position: Position) {
        self.visited.visit(position);
        self.path.push(position);
        self.events.visited(position);
    }

    /// Records DFS return from current dead end.
    pub fn backtrack(&mut self) {
        if let Some(position) = self.path.backtrack() {
            self.events.discarded(position);
        }
    }
}
