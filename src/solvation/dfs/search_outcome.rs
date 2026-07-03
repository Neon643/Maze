use crate::solvation::SolvationResult;
use crate::solvation::dfs::SearchContext;

/// Result of one immutable DFS recursion step.
pub enum SearchOutcome {
    /// DFS found final path.
    Found(SolvationResult),

    /// DFS did not find path in current branch and returns updated context.
    Searching(SearchContext),
}

impl SearchOutcome {
    /// Creates found search outcome.
    pub fn found(result: SolvationResult) -> Self {
        Self::Found(result)
    }

    /// Creates still-searching outcome.
    pub fn searching(context: SearchContext) -> Self {
        Self::Searching(context)
    }
}
