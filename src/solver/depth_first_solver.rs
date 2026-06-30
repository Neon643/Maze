use crate::domain::direction::Direction;
use crate::domain::maze::Maze;
use crate::domain::position::Position;
use crate::solver::search_result::SearchResult;
use crate::solver::search_step::SearchStep;
use std::collections::HashSet;

#[derive(Debug)]
pub struct DepthFirstSolver<'a> {
    maze: &'a Maze,
    start: Position,
    finish: Position,
    visited: HashSet<Position>,
    path: Vec<Position>,
    steps: Vec<SearchStep>,
}

impl<'a> DepthFirstSolver<'a> {
    pub fn new(maze: &'a Maze, start: Position, finish: Position) -> Self {
        Self {
            maze,
            start,
            finish,
            visited: HashSet::new(),
            path: Vec::new(),
            steps: Vec::new(),
        }
    }
    pub fn solve_with_steps(mut self) -> SearchResult {
        if !(self.maze.contains(self.start) && self.maze.contains(self.finish)) {
            SearchResult::new(None, self.steps)
        } else {
            if self.visit(self.start) {
                SearchResult::new(Some(self.path), self.steps)
            } else {
                SearchResult::new(None, self.steps)
            }
        }
    }

    fn visit(&mut self, current: Position) -> bool {
        self.visited.insert(current);
        self.path.push(current);
        self.steps.push(SearchStep::Entered(current));
        if current == self.finish {
            self.steps.push(SearchStep::Finished(self.path.clone()));
            true
        } else {
            for direction in Direction::ALL {
                if let Some(next) = self.maze.neighbor(current, direction)
                    && !self.visited.contains(&next)
                    && self.maze.has_passage(current, next)
                    && self.visit(next)
                {
                    return true;
                }
            }
            self.path.pop();
            self.steps.push(SearchStep::Backtracked);
            false
        }
    }
}
