use std::{iter::Peekable, str::Chars};

use aoc::Solution;
use itertools::Itertools;

struct Day3;

impl Solution<u64> for Day3 {
    type Input = &'static str;

    fn parse_input() -> Self::Input {
        include_str!("./3.txt")
    }

    fn part1(input: Self::Input) -> u64 {
        parse(input.chars())
            .filter_map(|instr| match instr {
                Instr::Mul(a, b) => Some(a * b),
                _ => None,
            })
            .sum()
    }

    fn part2(input: Self::Input) -> u64 {
        parse(input.chars())
            .fold((true, 0), |(doing, sum), instr| match instr {
                Instr::Mul(a, b) => {
                    if doing {
                        (doing, sum + a * b)
                    } else {
                        (doing, sum)
                    }
                }
                Instr::Do => (true, sum),
                Instr::Dont => (false, sum),
            })
            .1
    }
}

fn parse(chars: Chars<'_>) -> impl Iterator<Item = Instr> + use<'_> {
    Parser {
        chars: chars.peekable(),
    }
}

struct Parser<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Parser<'a> {
    type Item = Instr;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_instr()
    }
}

macro_rules! peek_consume {
    ($iter:expr, $($c:literal),+) => {
        $(
            if let $c = $iter.peek()? {
                $iter.next();
            } else {
                return Some(None);
            }
        )+
    }
}

impl<'a> Parser<'a> {
    fn next_instr(&mut self) -> Option<Instr> {
        while let Some(c) = self.chars.next() {
            match c {
                'd' => {
                    if let Some(instr) = self.parse_do_or_dont()? {
                        return Some(instr);
                    } else {
                        continue;
                    }
                }
                'm' => {
                    if let Some(instr) = self.parse_mul()? {
                        return Some(instr);
                    } else {
                        continue;
                    }
                }
                _ => continue,
            }
        }
        None
    }

    fn parse_do_or_dont(&mut self) -> Option<Option<Instr>> {
        peek_consume!(self.chars, 'o');
        match self.chars.peek()? {
            '(' => {
                let _ = self.chars.next();
                peek_consume!(self.chars, ')');
                Some(Some(Instr::Do))
            }
            'n' => {
                let _ = self.chars.next();
                peek_consume!(self.chars, '\'', 't', '(', ')');
                Some(Some(Instr::Dont))
            }
            _ => Some(None),
        }
    }

    fn parse_mul(&mut self) -> Option<Option<Instr>> {
        peek_consume!(self.chars, 'u', 'l', '(');
        let op1 = self
            .chars
            .peeking_take_while(|c| c.is_numeric())
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        peek_consume!(self.chars, ',');
        let op2 = self
            .chars
            .peeking_take_while(|c| c.is_numeric())
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
        peek_consume!(self.chars, ')');
        Some(Some(Instr::Mul(op1, op2)))
    }
}

#[derive(Debug)]
enum Instr {
    Mul(u64, u64),
    Do,
    Dont,
}

fn main() {
    Day3::run();
}
