use maze_algorithm::domain::maze::Maze;
use maze_algorithm::domain::maze_size::MazeSize;
use maze_algorithm::domain::position::Position;
use maze_algorithm::solvation::SolvationTask;
use maze_algorithm::solvation::dfs::{EmptySearch, SearchStage};

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
