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

    /// Returns new events collection with visited position recorded.
    pub fn visited(self, position: Position) -> Self {
        let mut events = self.events;
        events.push(SearchEvent::Visited(position));

        Self { events }
    }

    /// Returns new events collection with discarded position recorded.
    pub fn discarded(self, position: Position) -> Self {
        let mut events = self.events;
        events.push(SearchEvent::Discarded(position));

        Self { events }
    }
}
