use crate::domain::position::Position;
#[derive(Debug, Clone)]
pub enum GenerationStep {
    WalkStarted(Position),
    WalkMoved(Position),
    CycleErased(Vec<Position>),
    PassageOpened { from: Position, to: Position },
    CellAdded(Position),
}
