use mazeAlgorithm::domain::position::Position;
use mazeAlgorithm::solvation::SearchEvent;
use mazeAlgorithm::solvation::SolvationResult;

#[test]
fn solved_when_path_exists() {
    let start = Position::new(0, 0);
    let finish = Position::new(1, 1);

    let result = SolvationResult::new(
        Some(vec![start, finish]),
        vec![
            SearchEvent::Visited(start),
            SearchEvent::Visited(finish),
            SearchEvent::Finished(vec![start, finish]),
        ],
    );

    assert!(
        result.solved(),
        "result should be solved when final path exists"
    );
}

#[test]
fn not_solved_when_path_does_not_exist() {
    let start = Position::new(0, 0);

    let result = SolvationResult::new(
        None,
        vec![SearchEvent::Visited(start), SearchEvent::Discarded(start)],
    );

    assert!(
        !result.solved(),
        "result should not be solved when final path does not exist"
    );
}
