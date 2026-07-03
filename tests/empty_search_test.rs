use mazeAlgorithm::domain::maze::Maze;
use mazeAlgorithm::domain::maze_size::MazeSize;
use mazeAlgorithm::domain::position::Position;
use mazeAlgorithm::solvation::SolvationTask;
use mazeAlgorithm::solvation::dfs::{EmptySearch, SearchStage};

#[test]
fn builds_valid_search_context_from_valid_task() {
    let maze = Maze::new(MazeSize::new(2, 2));
    let task = SolvationTask::new(maze, Position::new(0, 0), Position::new(1, 1));

    let context = EmptySearch::new(task).eval();

    assert!(
        context.valid(),
        "empty search should build valid context from valid task"
    );
}

#[test]
fn builds_invalid_search_context_from_invalid_task() {
    let maze = Maze::new(MazeSize::new(2, 2));
    let task = SolvationTask::new(maze, Position::new(0, 0), Position::new(5, 5));

    let context = EmptySearch::new(task).eval();

    assert!(
        !context.valid(),
        "empty search should preserve invalid task state"
    );
}
