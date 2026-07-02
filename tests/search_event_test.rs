use mazeAlgorithm::domain::position::Position;
use mazeAlgorithm::solvation::SearchEvent;

#[test]
fn visited_event_stores_position() {
    let position = Position::new(0, 0);
    let event = SearchEvent::Visited(position);

    assert_eq!(
        event,
        SearchEvent::Visited(position),
        "visited event should store visited position"
    );
}

#[test]
fn discarded_event_stores_position() {
    let position = Position::new(1, 0);
    let event = SearchEvent::Discarded(position);

    assert_eq!(
        event,
        SearchEvent::Discarded(position),
        "discarded event should store discarded position"
    );
}

#[test]
fn finished_event_stores_final_path() {
    let start = Position::new(0, 0);
    let finish = Position::new(1, 1);
    let path = vec![start, finish];

    let event = SearchEvent::Finished(path.clone());

    assert_eq!(
        event,
        SearchEvent::Finished(path),
        "finished event should store final path"
    );
}
