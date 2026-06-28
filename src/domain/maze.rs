use crate::domain::cell::Cell;
use crate::domain::direction::Direction;
use crate::domain::position::Position;

#[derive(Debug, Clone)]
pub struct Maze {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        let initial_cell = Cell::new();
        let cells: Vec<Cell> = vec![initial_cell; width * height];
        Self {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn contains(&self, position: Position) -> bool {
        position.row() < self.height && position.col() < self.width
    }

    pub fn neighbor(&self, position: Position, direction: Direction) -> Option<Position> {
        let next = match direction {
            Direction::Up => {
                if position.row() == 0 {
                    return None;
                }
                Position::new(position.row() - 1, position.col())
            }
            Direction::Right => Position::new(position.row(), position.col() + 1),
            Direction::Down => Position::new(position.row() + 1, position.col()),
            Direction::Left => {
                if position.col() == 0 {
                    return None;
                }
                Position::new(position.row(), position.col() - 1)
            }
        };
        if self.contains(next) {
            Some(next)
        } else {
            None
        }
    }

    pub fn index(&self, position: Position) -> usize {
        position.row() * self.width + position.col()
    }

    pub fn open_passage(&mut self, from: Position, to: Position) -> bool {
        if !self.contains(from) || !self.contains(to) {
            return false;
        }
        for direction in Direction::ALL {
            if self.neighbor(from, direction) == Some(to) {
                let from_index = self.index(from);
                let to_index = self.index(to);
                self.cells[from_index].open(direction);
                self.cells[to_index].open(direction.opposite());
                return true;
            }
        }
        false
    }

    pub fn has_passage(&self, from: Position, to: Position) -> bool {
        if !self.contains(from) || !self.contains(to) {
            return false;
        }
        for direction in Direction::ALL {
            if self.neighbor(from, direction) == Some(to) {
                let from_index = self.index(from);
                let to_index = self.index(to);
                return self.cells[from_index].is_open(direction)
                    && self.cells[to_index].is_open(direction.opposite());
            }
        }
        false
    }

    pub fn positions(&self) -> Vec<Position> {
        let mut positions: Vec<Position> = Vec::new();

        for row in 0..self.height {
            for col in 0..self.width {
                positions.push(Position::new(row, col))
            }
        }
        positions
    }
}
