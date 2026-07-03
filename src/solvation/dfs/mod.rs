//! Depth-first maze solvation pipeline.

pub mod search_stage;
pub mod visited_positions;

pub use search_stage::SearchStage;
pub use visited_positions::VisitedPositions;
