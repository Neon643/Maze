use crate::solvation::SolvationResult;
use crate::solvation::dfs::SearchContext;

/// Intermediate state of DFS pipeline.
///
/// `Ready` means next stage may continue search.
/// `Failed` means previous stage already produced final failed result.
pub enum SearchState {
    /// Search may continue with prepared context.
    Ready(SearchContext),

    /// Search cannot continue.
    Failed(SolvationResult),
}

impl SearchState {
    /// Creates ready search state.
    pub fn ready(context: SearchContext) -> Self {
        Self::Ready(context)
    }

    /// Creates failed search state.
    pub fn failed(result: SolvationResult) -> Self {
        Self::Failed(result)
    }
}
