use std::collections::HashSet;
use std::sync::Mutex;

use crate::domain::direction::Direction;
use crate::domain::maze::Maze;
use crate::domain::position::Position;
use crate::solver::search_step::SearchStep;

#[derive(Debug)]
pub struct DepthFirstSolver;

impl DepthFirstSolver {
    pub fn solve(maze: &Maze, start: Position, finish: Position) -> Option<Vec<Position>> {
        if !(maze.contains(start) && maze.contains(finish)) {
            return None;
        } else {
            let mut visited: HashSet<Position> = HashSet::new();
            let mut path: Vec<Position> = Vec::new();
            if !(Self::dfs(maze, start, finish, &mut visited, &mut path)) {
                None
            } else {
                Some(path)
            }
        }
    }

    fn dfs(
        maze: &Maze,
        current: Position,
        finish: Position,
        visited: &mut HashSet<Position>,
        path: &mut Vec<Position>,
    ) -> bool {
        visited.insert(current);
        path.push(current);
        if current == finish {
            return true;
        } else {
            for direction in Direction::ALL {
                if let Some(next) = maze.neighbor(current, direction) {
                    if !visited.contains(&next) && maze.has_passage(current, next) {
                        if Self::dfs(maze, next, finish, visited, path) {
                            return true;
                        }
                    } else {
                    }
                }
            }
            path.pop();
            false
        }
    }

    pub fn solve_with_steps(
        maze: &Maze,
        start: Position,
        finish: Position,
    ) -> (Option<Vec<Position>>, Vec<SearchStep>) {
        if !(maze.contains(start) && maze.contains(finish)) {
            return (None, Vec::new());
        } else {
            let mut steps: Vec<SearchStep> = Vec::new();
            let mut visited: HashSet<Position> = HashSet::new();
            let mut path: Vec<Position> = Vec::new();
            if !(Self::dfs_with_steps(maze, start, finish, &mut visited, &mut path, &mut steps)) {
                (None, Vec::new())
            } else {
                (Some(path), steps)
            }
        }
    }

    fn dfs_with_steps(
        maze: &Maze,
        current: Position,
        finish: Position,
        visited: &mut HashSet<Position>,
        path: &mut Vec<Position>,
        steps: &mut Vec<SearchStep>,
    ) -> bool {
        visited.insert(current);
        path.push(current);
        if current == finish {
            steps.push(SearchStep::Finished(path.clone()));
            return true;
        } else {
            for direction in Direction::ALL {
                if let Some(next) = maze.neighbor(current, direction) {
                    if !visited.contains(&next) && maze.has_passage(current, next) {
                        steps.push(SearchStep::Entered(current));
                        if Self::dfs_with_steps(maze, next, finish, visited, path, steps) {
                            return true;
                        }
                    } else {
                    }
                }
            }
            path.pop();
            steps.push(SearchStep::Backtraced(current));
            false
        }
    }
}
