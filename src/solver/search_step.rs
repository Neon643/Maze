use crate::domain::position::Position;

pub enum SearchStep {
    Entered(Position),
    Backtraced(Position),
    Finished(Vec<Position>),
}
