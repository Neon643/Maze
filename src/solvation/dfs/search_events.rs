use crate::domain::position::Position;
use crate::solvation::SearchEvent;

/// Events recorded during DFS search.
///
/// `SearchEvents` hides event collection and exposes only domain actions
/// needed by DFS traversal.
#[derive(Default)]
pub struct SearchEvents {
    events: Vec<SearchEvent>,
}

impl SearchEvents {
    /// Creates empty search events collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records visited position.
    pub fn visited(&mut self, position: Position) {
        self.events.push(SearchEvent::Visited(position));
    }

    /// Records discarded position.
    pub fn discarded(&mut self, position: Position) {
        self.events.push(SearchEvent::Discarded(position));
    }
}
