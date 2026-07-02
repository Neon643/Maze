use mazeAlgorithm::domain::maze::Maze;
use mazeAlgorithm::domain::maze_size::MazeSize;
use mazeAlgorithm::domain::position::Position;
use mazeAlgorithm::solvation::SolvationTask;

#[test]
fn valid_when_start_and_finish_inside_maze() {
    let maze = Maze::new(MazeSize::new(2, 2));
    let task = SolvationTask::new(maze, Position::new(0, 0), Position::new(1, 1));

    assert!(
        task.valid(),
        "task should be valid when start and finish are inside maze"
    );
}

#[test]
fn invalid_when_start_outside_maze() {
    let maze = Maze::new(MazeSize::new(2, 2));
    let task = SolvationTask::new(maze, Position::new(2, 0), Position::new(1, 1));

    assert!(
        !task.valid(),
        "task should be invalid when start is outside maze"
    );
}

#[test]
fn invalid_when_finish_outside_maze() {
    let maze = Maze::new(MazeSize::new(2, 2));
    let task = SolvationTask::new(maze, Position::new(0, 0), Position::new(0, 2));

    assert!(
        !task.valid(),
        "task should be invalid when finish is outside maze"
    );
}
