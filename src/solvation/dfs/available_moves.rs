use crate::domain::position::Position;
use crate::solvation::dfs::SearchContext;

/// Passable DFS moves that were not reached before.
///
/// `AvailableMoves` selects next candidate positions from current DFS context
/// without changing search state.
pub struct AvailableMoves<'a> {
    context: &'a SearchContext,
    current: Position,
}

impl<'a> AvailableMoves<'a> {
    /// Creates available moves query for current position.
    pub fn new(context: &'a SearchContext, current: Position) -> Self {
        Self { context, current }
    }

    /// Returns passable and unreached neighbor positions.
    pub fn eval(self) -> Vec<Position> {
        self.context
            .passable_neighbors(self.current)
            .into_iter()
            .filter(|position| !self.context.reached(*position))
            .collect()
    }
}
