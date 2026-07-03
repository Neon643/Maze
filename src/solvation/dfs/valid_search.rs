use crate::solvation::SolvationResult;
use crate::solvation::dfs::{SearchContext, SearchStage, SearchState};

/// Validation stage of DFS pipeline.
///
/// `ValidSearch` checks that start and finish positions belong to maze.
/// It does not check whether path between them exists.
pub struct ValidSearch<S> {
    origin: S,
}

impl<S> ValidSearch<S> {
    /// Creates validation stage over previous DFS stage.
    pub fn new(origin: S) -> Self {
        Self { origin }
    }
}

impl<S> SearchStage for ValidSearch<S>
where
    S: SearchStage<Output = SearchContext>,
{
    type Output = SearchState;

    fn eval(self) -> Self::Output {
        let context = self.origin.eval();

        if context.valid() {
            SearchState::ready(context)
        } else {
            SearchState::failed(SolvationResult::new(None, Vec::new()))
        }
    }
}
