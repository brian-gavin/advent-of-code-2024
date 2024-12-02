use std::iter;

use aoc::Solution;
use itertools::Itertools;

struct Day1;

impl Solution<u64> for Day1 {
    type Input = (Vec<u64>, Vec<u64>);

    fn parse_input() -> Self::Input {
        include_str!("./1.txt")
            .lines()
            .map(|l| -> (u64, u64) {
                l.split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .unzip()
    }

    fn part1(mut v: Self::Input) -> u64 {
        v.0.sort();
        v.1.sort();
        iter::zip(v.0.into_iter(), v.1.into_iter())
            .map(|(n1, n2)| n1.abs_diff(n2))
            .sum()
    }

    fn part2(v: Self::Input) -> u64 {
        let occur = v.1.into_iter().counts();
        v.0.into_iter()
            .map(|n| {
                let occur = *occur.get(&n).unwrap_or(&0) as u64;
                n * occur
            })
            .sum()
    }
}

fn main() {
    Day1::run();
}
