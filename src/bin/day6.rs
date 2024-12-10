use std::collections::HashSet;

use aoc::{
    grid::{Coord, Grid},
    Solution,
};
use itertools::Itertools;
use rayon::prelude::*;

struct Day6;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum State {
    Guard(Direction),
    Obstacle,
    Empty,
}

impl From<char> for State {
    fn from(value: char) -> Self {
        use State::*;
        if let Ok(value) = Direction::try_from(value) {
            return Guard(value);
        }
        match value {
            '#' => Obstacle,
            '.' => Empty,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Direction::*;
        match value {
            '^' => Ok(North),
            '>' => Ok(East),
            'v' => Ok(South),
            '<' => Ok(West),
            _ => Err(()),
        }
    }
}

impl Direction {
    fn coord_fn(self) -> impl Fn(Coord, isize) -> Coord {
        match self {
            Direction::North => Coord::north,
            Direction::East => Coord::east,
            Direction::South => Coord::south,
            Direction::West => Coord::west,
        }
    }

    fn turn_90(self) -> Direction {
        use Direction::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}

impl Solution<usize> for Day6 {
    type Input = (Grid<State>, Coord);

    fn parse_input() -> Self::Input {
        let grid = Grid::from_iter(include_str!("./6.txt").lines().enumerate().flat_map(
            |(row, l)| {
                l.char_indices().map(move |(col, c)| {
                    let state = State::from(c);
                    let coord = Coord::from((row as isize, col as isize));
                    (coord, state)
                })
            },
        ));
        let guard_start = grid
            .iter()
            .find_map(|(coord, state)| {
                if let State::Guard(_) = state {
                    Some(*coord)
                } else {
                    None
                }
            })
            .unwrap();
        (grid, guard_start)
    }

    fn part1((grid, guard_pos): Self::Input) -> usize {
        let mut visited = HashSet::new();
        visited.insert(guard_pos);
        traverse(grid, guard_pos, |coord, _| {
            let _ = visited.insert(coord);
            false
        });
        visited.len()
    }

    fn part2((grid, guard_pos): Self::Input) -> usize {
        let visited = {
            let mut visited = HashSet::new();
            traverse(grid.clone(), guard_pos, |coord, _| {
                visited.insert(coord);
                false
            });
            visited
        };
        let empty_coords = visited
            .iter()
            .filter_map(|coord| {
                if let Some(State::Empty) = grid.at(*coord) {
                    Some(*coord)
                } else {
                    None
                }
            })
            .collect_vec();
        rayon::ThreadPoolBuilder::new()
            .num_threads(empty_coords.len())
            .build_global()
            .unwrap();
        empty_coords
            .into_par_iter()
            .filter(|pos| creates_cycle(grid.clone(), guard_pos, *pos))
            .count()
    }
}

fn creates_cycle(mut grid: Grid<State>, guard_pos: Coord, pos: Coord) -> bool {
    let _ = grid.insert(pos, State::Obstacle);
    let mut visited = HashSet::new();
    traverse(grid, guard_pos, |coord, state| {
        !visited.insert((coord, state))
    })
    .is_some()
}

fn traverse<F>(mut grid: Grid<State>, mut guard_pos: Coord, mut on_visit: F) -> Option<Coord>
where
    F: FnMut(Coord, State) -> bool,
{
    while let Some(guard_state @ State::Guard(dir)) = grid.at(guard_pos).cloned() {
        if on_visit(guard_pos, guard_state) {
            return Some(guard_pos);
        }
        let next_coord = dir.coord_fn()(guard_pos, 1);
        match grid.at(next_coord).cloned() {
            Some(State::Empty) => {
                grid.swap(next_coord, guard_pos);
                guard_pos = next_coord;
            }
            Some(State::Obstacle) => {
                let _ =
                    std::mem::replace(grid.at_mut(guard_pos).unwrap(), State::Guard(dir.turn_90()));
            }
            Some(State::Guard(_)) => unreachable!(),
            None => break,
        }
    }
    None
}

#[allow(dead_code)]
fn part2_debug(grid: Grid<State>, guard_pos: Coord) {
    [
        guard_pos.west(1),
        guard_pos.south(1).east(2),
        guard_pos.south(1).east(3),
        guard_pos.south(2).west(3),
        guard_pos.south(2).west(1),
        guard_pos.south(3).east(3),
    ]
    .into_iter()
    .for_each(|coord| {
        dbg!(creates_cycle(grid.clone(), guard_pos, dbg!(coord)));
    })
}

fn main() {
    Day6::run()
}
