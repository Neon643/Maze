use maze_algorithm::domain::maze::Maze;
use maze_algorithm::domain::maze_size::MazeSize;
use maze_algorithm::domain::position::Position;
use maze_algorithm::solvation::SolvationTask;
use maze_algorithm::solvation::dfs::{EmptySearch, SearchStage, SearchState, ValidSearch};

#[test]
fn returns_ready_state_when_task_is_valid() {
    let maze = Maze::new(MazeSize::new(2, 2));
    let task = SolvationTask::new(maze, Position::new(0, 0), Position::new(1, 1));

    let state = ValidSearch::new(EmptySearch::new(task)).eval();

    assert!(
        matches!(state, SearchState::Ready(_)),
        "valid search should return ready state for valid task"
    );
}

#[test]
fn returns_failed_state_when_task_is_invalid() {
    let maze = Maze::new(MazeSize::new(2, 2));
    let task = SolvationTask::new(maze, Position::new(0, 0), Position::new(9, 9));

    let state = ValidSearch::new(EmptySearch::new(task)).eval();

    assert!(
        matches!(state, SearchState::Failed(_)),
        "valid search should return failed state for invalid task"
    );
}
