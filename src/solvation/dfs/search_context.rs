use crate::domain::position::Position;
use crate::solvation::SolvationResult;
use crate::solvation::SolvationTask;
use crate::solvation::dfs::SearchTrace;

/// Prepared DFS search context.
///
/// `SearchContext` keeps solvation task together with immutable DFS trace.
/// Every search action returns new context.
pub struct SearchContext {
    task: SolvationTask,
    trace: SearchTrace,
}

impl SearchContext {
    /// Creates DFS context from solvation task and empty trace.
    pub fn new(task: SolvationTask) -> Self {
        Self {
            task,
            trace: SearchTrace::new(),
        }
    }

    /// Checks whether context contains executable solvation task.
    pub fn valid(&self) -> bool {
        self.task.valid()
    }

    /// Checks whether position was already reached by DFS.
    pub fn reached(&self, position: Position) -> bool {
        self.trace.reached(position)
    }

    /// Returns new context with entered position recorded.
    pub fn entered(self, position: Position) -> Self {
        Self {
            task: self.task,
            trace: self.trace.entered(position),
        }
    }

    /// Returns new context with current dead end discarded.
    pub fn backtracked(self) -> Self {
        Self {
            task: self.task,
            trace: self.trace.backtracked(),
        }
    }
    /// Returns positions reachable through opened passages.
    pub fn passable_neighbors(&self, position: Position) -> Vec<Position> {
        self.task.passable_neighbors(position)
    }
    /// Converts context into successful solvation result.
    pub fn succeeded(self) -> SolvationResult {
        self.trace.succeeded()
    }

    /// Converts context into failed solvation result.
    pub fn failed(self) -> SolvationResult {
        self.trace.failed()
    }
}
