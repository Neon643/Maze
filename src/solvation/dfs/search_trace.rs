use crate::domain::position::Position;
use crate::solvation::dfs::{SearchEvents, SearchPath, VisitedPositions};

/// DFS trace composed from visited positions, current path and search events.
///
/// `SearchTrace` coordinates trace objects without exposing their internal
/// collections. Every search action returns new trace.
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

    /// Returns new trace with DFS entrance into position recorded.
    pub fn entered(self, position: Position) -> Self {
        Self {
            visited: self.visited.visited(position),
            path: self.path.pushed(position),
            events: self.events.visited(position),
        }
    }

    /// Returns new trace with DFS return from current dead end recorded.
    pub fn backtracked(self) -> Self {
        let (path, discarded) = self.path.backtracked();

        let events = match discarded {
            Some(position) => self.events.discarded(position),
            None => self.events,
        };

        Self {
            visited: self.visited,
            path,
            events,
        }
    }
}
