use crate::solvation::SolvationTask;
use crate::solvation::dfs::{SearchContext, SearchStage};

/// First stage of DFS pipeline.
///
/// `EmptySearch` turns solvation task into initial DFS context.
pub struct EmptySearch {
    task: SolvationTask,
}

impl EmptySearch {
    /// Creates first DFS pipeline stage.
    pub fn new(task: SolvationTask) -> Self {
        Self { task }
    }
}

impl SearchStage for EmptySearch {
    type Output = SearchContext;

    fn eval(self) -> Self::Output {
        SearchContext::new(self.task)
    }
}
