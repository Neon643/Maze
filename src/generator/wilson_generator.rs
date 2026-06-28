use std::collections::HashSet;

use crate::domain::direction::Direction;
use crate::domain::maze::Maze;
use crate::domain::position::Position;
use crate::generator::generation_step::GenerationStep;

#[derive(Debug)]
pub struct WilsonGenerator {
    steps: Vec<GenerationStep>,
}

impl WilsonGenerator {
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    pub fn steps(&self) -> &[GenerationStep] {
        &self.steps
    }

    pub fn generate(&mut self, width: usize, height: usize) -> Maze {
        let mut maze = Maze::new(width, height);
        let positions = maze.positions();
        let mut in_maze: HashSet<Position> = HashSet::new();
        let start_index = rand::random_range(0..positions.len());
        let start = positions[start_index];
        in_maze.insert(start);
        while in_maze.len() < positions.len() {
            let candidates: Vec<Position> = positions
                .iter()
                .copied()
                .filter(|position| !in_maze.contains(position))
                .collect();
            let walk_start_index = rand::random_range(0..candidates.len());
            let walk_start = candidates[walk_start_index];
            let mut path = vec![walk_start];
            let mut current = walk_start;
            // println!("{:?}", start);

            while !in_maze.contains(&current) {
                let neighbors: Vec<Position> = Direction::ALL
                    .iter()
                    .copied()
                    .filter_map(|direction| maze.neighbor(current, direction))
                    .collect();
                let random_neighbor_index = rand::random_range(0..neighbors.len());
                let random_neighbor = neighbors[random_neighbor_index];
                if let Some(index) = path
                    .iter()
                    .position(|position| *position == random_neighbor)
                {
                    path.truncate(index + 1);
                } else {
                    path.push(random_neighbor);
                }
                // println!("root : {:?}", start);
                // println!("walk_start : {:?}", walk_start);
                // println!("path : {:?}", path);
                current = random_neighbor;
            }
            for pair in path.windows(2) {
                let from = pair[0];
                let to = pair[1];
                maze.open_passage(from, to);
                self.steps.push(GenerationStep::PassageOpened { from, to });
            }
            for position in &path {
                in_maze.insert(*position);
            }
        }
        maze
    }
}
