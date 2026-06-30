use crate::domain::position::Position;
#[derive(Debug, Copy, Clone)]
pub struct RandomPositions;
impl RandomPositions {
    pub fn new() -> Self {
        Self
    }
    pub fn pick(&self, positions: &[Position]) -> Position {
        let index = rand::random_range(0..positions.len());
        positions[index]
    }
}
