#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MazeSize {
    width: usize,
    height: usize,
}

impl MazeSize {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
