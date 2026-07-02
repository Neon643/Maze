use crate::domain::position::Position;
use crate::solvation::search_event::SearchEvent;

/// Result produced by maze solvation strategy.
///
/// Contains final path when path was found and search events
/// recorded during algorithm execution.
pub struct SolvationResult {
    path: Option<Vec<Position>>,
    events: Vec<SearchEvent>,
}

impl SolvationResult {
    /// Creates solvation result from optional path and search events.
    pub fn new(path: Option<Vec<Position>>, events: Vec<SearchEvent>) -> Self {
        Self { path, events }
    }
    /// Checks whether solvation found final path.
    pub fn solved(&self) -> bool {
        self.path.is_some()
    }
}
