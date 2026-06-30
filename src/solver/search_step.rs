use crate::domain::position::Position;
#[derive(Debug)]
pub enum SearchStep {
    Entered(Position),
    Backtracked,
    Finished(Vec<Position>),
}
