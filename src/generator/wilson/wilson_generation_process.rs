use crate::domain::maze::Maze;
use crate::domain::maze_size::MazeSize;
use crate::domain::position::Position;
use crate::generator::random_positions::RandomPositions;
use crate::generator::wilson::generation_step::GenerationStep;
use crate::generator::wilson::wilson_generation::WilsonGeneration;
use crate::generator::wilson::wilson_tree::WilsonTree;
use crate::generator::wilson::wilson_walk::WilsonWalk;

pub struct WilsonGenerationProcess {
    maze: Maze,
    steps: Vec<GenerationStep>,
    positions: Vec<Position>,
    tree: WilsonTree,
    random: RandomPositions,
}

impl WilsonGenerationProcess {
    pub fn new(size: MazeSize, random: RandomPositions) -> Self {
        let maze = Maze::new(size);
        let positions = maze.positions();
        let start = random.pick(&positions);
        let tree = WilsonTree::new(start);

        Self {
            maze,
            steps: Vec::new(),
            positions,
            tree,
            random,
        }
    }
    pub fn run(mut self) -> WilsonGeneration {
        while !self.is_finished() {
            self.grow_tree();
        }

        WilsonGeneration::new(self.maze, self.steps)
    }
    fn is_finished(&self) -> bool {
        self.tree.len() >= self.positions.len()
    }

    fn grow_tree(&mut self) {
        let candidates = self.tree.missing_from(&self.positions);
        let walk_start = self.random.pick(&candidates);
        let walk = self.random_walk_from(walk_start);

        walk.carve_into(&mut self.maze, &mut self.steps);
        self.tree.include_walk(&walk);
    }

    fn random_walk_from(&self, start: Position) -> WilsonWalk {
        let mut walk = WilsonWalk::new(start);

        while !self.tree.contains(walk.current()) {
            let neighbors = self.maze.neighbors(walk.current());
            let next = self.random.pick(&neighbors);
            walk.step_to(next);
        }

        walk
    }
}
