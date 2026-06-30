use crate::domain::maze_size::MazeSize;
use crate::generator::random_positions::RandomPositions;
use crate::generator::wilson::wilson_generation::WilsonGeneration;
use crate::generator::wilson::wilson_generation_process::WilsonGenerationProcess;

#[derive(Debug)]
pub struct WilsonGenerator {
    size: MazeSize,
    random: RandomPositions,
}

impl WilsonGenerator {
    pub fn new(size: MazeSize, random: RandomPositions) -> Self {
        Self { size, random }
    }

    pub fn generate(&self) -> WilsonGeneration {
        WilsonGenerationProcess::new(self.size, self.random).run()
    }
}
