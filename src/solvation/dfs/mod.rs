//! Depth-first maze solvation pipeline.

pub mod empty_search;
pub mod search_context;
pub mod search_events;
pub mod search_path;
pub mod search_stage;
pub mod search_trace;
pub mod visited_positions;

pub use empty_search::EmptySearch;
pub use search_context::SearchContext;
pub use search_events::SearchEvents;
pub use search_path::SearchPath;
pub use search_stage::SearchStage;
pub use search_trace::SearchTrace;
pub use visited_positions::VisitedPositions;
