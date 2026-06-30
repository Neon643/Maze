use crate::domain::position::Position;
#[derive(Debug, Clone)]
pub enum GenerationStep {
    PassageOpened { from: Position, to: Position },
}
