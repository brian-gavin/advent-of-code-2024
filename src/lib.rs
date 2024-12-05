use std::{env::args, fmt::Display, time::Instant};

pub mod grid;

pub trait Solution<Output: Display> {
    type Input;

    fn parse_input() -> Self::Input;
    fn part1(input: Self::Input) -> Output;
    fn part2(input: Self::Input) -> Output;

    fn run() {
        let start = Instant::now();
        let input = Self::parse_input();
        let out = match part() {
            Part::One => Self::part1(input),
            Part::Two => Self::part2(input),
        };
        println!("{}", out);
        println!("{:?}", start.elapsed())
    }
}

enum Part {
    One,
    Two,
}

fn part() -> Part {
    match args().last() {
        Some(p) if p == "1" => Part::One,
        Some(p) if p == "2" => Part::Two,
        _ => bail("part not specified"),
    }
}

fn bail(msg: &str) -> ! {
    eprintln!("{}", msg);
    std::process::exit(1)
}
