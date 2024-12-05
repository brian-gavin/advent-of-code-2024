use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    ops::Not,
};

use aoc::Solution;

struct Day5;

type RulesMap = HashMap<usize, HashSet<usize>>;

impl Solution<usize> for Day5 {
    type Input = (HashMap<usize, HashSet<usize>>, Vec<Vec<usize>>);

    fn parse_input() -> Self::Input {
        let mut lines = include_str!("./5.txt").lines();
        let ordering_rules = lines
            .by_ref()
            .map_while(|line| {
                line.split_once("|")
                    .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
            })
            .fold(RulesMap::default(), |mut m, (before, after)| {
                m.entry(before).or_default().insert(after);
                m
            });
        let pages = lines
            .map(|line| {
                line.split(",")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();
        (ordering_rules, pages)
    }

    fn part1((ordering, pages): Self::Input) -> usize {
        pages
            .into_iter()
            .filter(|pages| {
                pages.is_sorted_by(|a, b| compare_rules(&ordering)(a, b) != Ordering::Greater)
            })
            .filter_map(|pages| pages.get(pages.len() / 2).cloned())
            .sum()
    }

    fn part2((ordering, pages): Self::Input) -> usize {
        // wrong: 11451
        pages
            .into_iter()
            .filter_map(|mut pages| {
                let cmp = compare_rules(&ordering);
                pages
                    .is_sorted_by(|a, b| not_greater(cmp(a, b)))
                    .not()
                    .then(|| {
                        pages.sort_by(cmp);
                        pages
                    })
            })
            .filter_map(|pages| pages.get(pages.len() / 2).cloned())
            .sum()
    }
}

fn not_greater(o: Ordering) -> bool {
    o != Ordering::Greater
}

fn compare_rules(ordering_rules: &RulesMap) -> impl Fn(&usize, &usize) -> Ordering + use<'_> {
    |a, b| -> Ordering {
        ordering_rules
            .get(a)
            .and_then(|set| set.contains(b).then_some(Ordering::Less))
            .unwrap_or(Ordering::Greater)
    }
}

fn main() {
    Day5::run();
}
