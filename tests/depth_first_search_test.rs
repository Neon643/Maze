use mazeAlgorithm::domain::maze::Maze;
use mazeAlgorithm::domain::maze_size::MazeSize;
use mazeAlgorithm::domain::position::Position;
use mazeAlgorithm::solvation::SolvationTask;
use mazeAlgorithm::solvation::dfs::{DepthFirstSearch, EmptySearch, SearchStage, ValidSearch};

#[test]
fn solves_maze_when_path_exists() {
    let mut maze = Maze::new(MazeSize::new(2, 2));
    let start = Position::new(0, 0);
    let finish = Position::new(1, 1);
    let right = Position::new(0, 1);

    assert!(maze.open_passage(start, right));
    assert!(maze.open_passage(right, finish));

    let task = SolvationTask::new(maze, start, finish);

    let result = DepthFirstSearch::new(ValidSearch::new(EmptySearch::new(task))).eval();

    assert!(
        result.solved(),
        "DFS should solve maze when opened path exists"
    );
}

#[test]
fn does_not_solve_maze_when_path_does_not_exist() {
    let maze = Maze::new(MazeSize::new(2, 2));
    let start = Position::new(0, 0);
    let finish = Position::new(1, 1);
    let task = SolvationTask::new(maze, start, finish);

    let result = DepthFirstSearch::new(ValidSearch::new(EmptySearch::new(task))).eval();

    assert!(
        !result.solved(),
        "DFS should not solve maze when no opened path exists"
    );
}

#[test]
fn does_not_solve_invalid_task() {
    let maze = Maze::new(MazeSize::new(2, 2));
    let start = Position::new(0, 0);
    let finish = Position::new(9, 9);
    let task = SolvationTask::new(maze, start, finish);

    let result = DepthFirstSearch::new(ValidSearch::new(EmptySearch::new(task))).eval();

    assert!(!result.solved(), "DFS should not solve invalid task");
}
