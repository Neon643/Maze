use crate::domain::position::Position;
use crate::solvation::SolvationResult;
use crate::solvation::dfs::{
    AvailableMoves, SearchContext, SearchOutcome, SearchStage, SearchState,
};

/// DFS traversal stage.
///
/// `DepthFirstSearch` executes recursive immutable DFS over validated search
/// state and produces final solvation result.
pub struct DepthFirstSearch<S> {
    origin: S,
}

impl<S> DepthFirstSearch<S> {
    /// Creates DFS traversal stage over previous pipeline stage.
    pub fn new(origin: S) -> Self {
        Self { origin }
    }

    fn visit(current: Position, context: SearchContext) -> SearchOutcome {
        let context = context.entered(current);

        if context.finishes_at(current) {
            SearchOutcome::found(context.succeeded())
        } else {
            let moves = AvailableMoves::new(&context, current).eval();

            Self::visit_moves(&moves, context)
        }
    }

    fn visit_moves(moves: &[Position], context: SearchContext) -> SearchOutcome {
        match moves.split_first() {
            Some((next, rest)) if context.reached(*next) => Self::visit_moves(rest, context),
            Some((next, rest)) => match Self::visit(*next, context) {
                SearchOutcome::Found(result) => SearchOutcome::found(result),
                SearchOutcome::Searching(context) => Self::visit_moves(rest, context),
            },
            None => SearchOutcome::searching(context.backtracked()),
        }
    }
}

impl<S> SearchStage for DepthFirstSearch<S>
where
    S: SearchStage<Output = SearchState>,
{
    type Output = SolvationResult;

    fn eval(self) -> Self::Output {
        match self.origin.eval() {
            SearchState::Ready(context) => match Self::visit(context.starts_at(), context) {
                SearchOutcome::Found(result) => result,
                SearchOutcome::Searching(context) => context.failed(),
            },
            SearchState::Failed(result) => result,
        }
    }
}
