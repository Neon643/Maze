use crate::domain::position::Position;
/// Event produced by maze path search algorithm.
///
/// `SearchEvent` describes algorithm progress in a generic way,
/// so it can be reused by different solvation strategies:
/// DFS, BFS, Dijkstra, A* and others.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchEvent {
    /// Position was reached or processed by search algorithm.
    Visited(Position),
    /// Position was rejected, rolled back from, or removed from current candidate path.
    Discarded(Position),
    /// Search finished and produced final path.
    Finished(Vec<Position>),
}
