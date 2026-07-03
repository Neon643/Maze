use maze_algorithm::domain::maze::Maze;
use maze_algorithm::domain::maze_size::MazeSize;
use maze_algorithm::domain::position::Position;
use maze_algorithm::solvation::SolvationTask;
use maze_algorithm::solvation::dfs::{AvailableMoves, EmptySearch, SearchStage};

#[test]
fn returns_no_moves_when_passages_are_closed() {
    let maze = Maze::new(MazeSize::new(2, 2));
    let start = Position::new(0, 0);
    let finish = Position::new(1, 1);
    let task = SolvationTask::new(maze, start, finish);

    let context = EmptySearch::new(task).eval().entered(start);
    let moves = AvailableMoves::new(&context, start).eval();

    assert!(
        moves.is_empty(),
        "available moves should be empty when all passages are closed"
    );
}

#[test]
fn returns_passable_unreached_neighbor() {
    let mut maze = Maze::new(MazeSize::new(2, 2));
    let start = Position::new(0, 0);
    let right = Position::new(0, 1);
    let finish = Position::new(1, 1);

    assert!(maze.open_passage(start, right));

    let task = SolvationTask::new(maze, start, finish);
    let context = EmptySearch::new(task).eval().entered(start);

    let moves = AvailableMoves::new(&context, start).eval();

    assert_eq!(
        moves,
        vec![right],
        "available moves should contain opened and unreached neighbor"
    );
}

#[test]
fn skips_reached_neighbor() {
    let mut maze = Maze::new(MazeSize::new(2, 2));
    let start = Position::new(0, 0);
    let right = Position::new(0, 1);
    let down = Position::new(1, 0);
    let finish = Position::new(1, 1);

    assert!(maze.open_passage(start, right));
    assert!(maze.open_passage(start, down));

    let task = SolvationTask::new(maze, start, finish);
    let context = EmptySearch::new(task).eval().entered(start).entered(right);

    let moves = AvailableMoves::new(&context, start).eval();

    assert_eq!(
        moves,
        vec![down],
        "available moves should skip already reached neighbor"
    );
}
