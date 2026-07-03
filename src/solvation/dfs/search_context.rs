use crate::solvation::SolvationTask;

/// Prepared DFS search context.
///
/// `SearchContext` is created by the first DFS pipeline stage and then passed
/// through validation and traversal stages.
pub struct SearchContext {
    task: SolvationTask,
}

impl SearchContext {
    /// Creates DFS context from solvation task.
    pub fn new(task: SolvationTask) -> Self {
        Self { task }
    }

    /// Checks whether context contains executable solvation task.
    pub fn valid(&self) -> bool {
        self.task.valid()
    }
}
