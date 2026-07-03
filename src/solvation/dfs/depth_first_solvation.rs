use crate::solvation::dfs::SearchStage;
use crate::solvation::{Solvation, SolvationResult};

/// DFS solvation strategy.
///
/// `DepthFirstSolvation` is final wrapper over assembled DFS pipeline.
/// It adapts DFS pipeline to common `Solvation` contract.
pub struct DepthFirstSolvation<S> {
    search: S,
}

impl<S> DepthFirstSolvation<S> {
    /// Creates DFS solvation from assembled search pipeline.
    pub fn new(search: S) -> Self {
        Self { search }
    }
}

impl<S> Solvation for DepthFirstSolvation<S>
where
    S: SearchStage<Output = SolvationResult>,
{
    type Result = SolvationResult;

    fn eval(self) -> Self::Result {
        self.search.eval()
    }
}
