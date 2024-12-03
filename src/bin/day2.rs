use aoc::Solution;

struct Day2;

impl Solution<usize> for Day2 {
    type Input = Vec<Vec<usize>>;

    fn parse_input() -> Self::Input {
        include_str!("./2.txt")
            .lines()
            .map(|l| l.split_whitespace().map(|s| s.parse().unwrap()).collect())
            .collect()
    }

    fn part1(input: Self::Input) -> usize {
        input.into_iter().filter(|v| safe(v, &mut None)).count()
    }

    fn part2(input: Self::Input) -> usize {
        input
            .into_iter()
            .filter(|v| {
                if safe(v, &mut None) {
                    return true;
                }
                for i in 0..v.len() {
                    let mut v = v.clone();
                    v.remove(i);
                    if safe(&v, &mut None) {
                        return true;
                    }
                }
                return false;
            })
            .count()
    }
}

#[derive(Debug)]
enum Ordering {
    Increasing,
    Decreasing,
}

fn safe(v: &[usize], o: &mut Option<Ordering>) -> bool {
    match v {
        [_] | [] => true,
        [h, t @ ..] => check(h, &t[0], o) && safe(t, o),
    }
}

fn check(h: &usize, t0: &usize, o: &mut Option<Ordering>) -> bool {
    let diff = h.abs_diff(*t0);
    if diff > 3 || diff < 1 {
        return false;
    }
    match o.get_or_insert_with(|| {
        if h < t0 {
            Ordering::Increasing
        } else {
            Ordering::Decreasing
        }
    }) {
        Ordering::Increasing if h > t0 => {
            return false;
        }
        Ordering::Decreasing if h < t0 => {
            return false;
        }
        _ => (),
    };
    true
}

fn main() {
    Day2::run()
}
