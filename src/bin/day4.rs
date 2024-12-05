use aoc::{
    grid::{Coord, Grid},
    Solution,
};

struct Day4;

impl Solution<usize> for Day4 {
    type Input = Grid<char>;

    fn parse_input() -> Self::Input {
        Grid::from_iter(
            include_str!("./4.txt")
                .lines()
                .enumerate()
                .flat_map(|(col, l)| {
                    l.char_indices()
                        .map(move |(row, c)| ((row as isize, col as isize).into(), c))
                }),
        )
    }

    fn part1(input: Self::Input) -> usize {
        input
            .iter()
            .filter_map(|(start, c)| {
                if *c == 'X' {
                    Some(
                        [
                            Coord::north,
                            Coord::northeast,
                            Coord::east,
                            Coord::southeast,
                            Coord::south,
                            Coord::southwest,
                            Coord::west,
                            Coord::northwest,
                        ]
                        .into_iter()
                        .filter(|direction| xmas_searcher(&input, *start, direction))
                        .count(),
                    )
                } else {
                    None
                }
            })
            .sum()
    }

    fn part2(input: Self::Input) -> usize {
        input
            .iter()
            .filter(|(_, c)| **c == 'A')
            .filter(|(a, _)| crossmas_searcher(&input, **a))
            .count()
    }
}

fn xmas_searcher(grid: &Grid<char>, x: Coord, direction: impl Fn(Coord, isize) -> Coord) -> bool {
    let Some('M') = grid.at(direction(x, 1)) else {
        return false;
    };
    let Some('A') = grid.at(direction(x, 2)) else {
        return false;
    };
    let Some('S') = grid.at(direction(x, 3)) else {
        return false;
    };
    true
}

fn crossmas_searcher(grid: &Grid<char>, a: Coord) -> bool {
    /*
    M   S
     A   A
      S   M
     */
    (match (grid.at(a.northwest(1)), grid.at(a.southeast(1))) {
        (Some('M'), Some('S')) => true,
        (Some('S'), Some('M')) => true,
        _ => false,
    })
    &&
    /*
      M   S
     A   A
    S   M
     */
    (match (grid.at(a.northeast(1)), grid.at(a.southwest(1))) {
        (Some('M'), Some('S')) => true,
        (Some('S'), Some('M')) => true,
        _ => false,
    })
}

fn main() {
    Day4::run();
}
