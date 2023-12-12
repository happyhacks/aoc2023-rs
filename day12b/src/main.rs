use std::{collections::HashMap, fmt};

use itertools::{repeat_n, Itertools};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Spring {
    Damaged,
    Operational,
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParseError {
    InvalidChar,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidChar => write!(f, "Invalid character encountered"),
        }
    }
}
impl TryFrom<char> for Spring {
    type Error = ParseError;
    fn try_from(c: char) -> Result<Self, ParseError> {
        match c {
            '.' => Ok(Self::Operational),
            '#' => Ok(Self::Damaged),
            '?' => Ok(Self::Unknown),
            _ => Err(ParseError::InvalidChar),
        }
    }
}

#[derive(Debug, Clone)]
struct Record {
    row: Vec<Spring>,
    damaged: Vec<usize>,
    h: HashMap<(usize, usize, usize), usize>,
}

impl Record {
    fn count(&mut self, rx: usize, dx: usize, cdmg: usize) -> usize {
        let k = (rx, dx, cdmg);
        if self.h.contains_key(&k) {
            return *self.h.get(&k).unwrap();
        }
        if rx == self.row.len() {
            return match (dx, cdmg) {
                (x, 0) if x == self.damaged.len() => 1,
                (x, y) if x == self.damaged.len() - 1 && y == self.damaged[x] => 1,
                _ => 0,
            };
        }
        let mut count = 0;
        match self.row[rx] {
            Spring::Damaged => {
                count += self.count(rx + 1, dx, cdmg + 1);
            }
            Spring::Operational => {
                if cdmg == 0 {
                    count += self.count(rx + 1, dx, 0)
                }
                if dx < self.damaged.len() && self.damaged[dx] == cdmg {
                    count += self.count(rx + 1, dx + 1, 0)
                }
            }
            Spring::Unknown => {
                count += self.count(rx + 1, dx, cdmg + 1); // assume Damaged
                                                           // assume Operational
                if cdmg == 0 {
                    count += self.count(rx + 1, dx, 0)
                }
                if dx < self.damaged.len() && self.damaged[dx] == cdmg {
                    count += self.count(rx + 1, dx + 1, 0)
                }
            }
        }
        self.h.insert(k, count);
        count
    }
}

fn main() {
    let mut records: Vec<Record> = include_str!("/tmp/input.txt")
        .trim_end()
        .lines()
        .map(|l| {
            let mut parts = l.split_ascii_whitespace();
            Record {
                row: Itertools::intersperse(
                    repeat_n(
                        parts
                            .next()
                            .unwrap()
                            .chars()
                            .map(|c| Spring::try_from(c).unwrap())
                            .collect(),
                        5,
                    ),
                    vec![Spring::Unknown],
                )
                .flat_map(|x| x)
                .collect(),
                damaged: repeat_n(
                    parts
                        .next()
                        .unwrap()
                        .split(",")
                        .map(|n| n.parse().unwrap())
                        .collect(),
                    5,
                )
                .flat_map(|x: Vec<usize>| x)
                .collect(),
                h: HashMap::new(),
            }
        })
        .collect();
    // dbg!(records);
    println!(
        "{}",
        records
            .iter_mut()
            .map(|r: &mut Record| r.count(0, 0, 0))
            .sum::<usize>()
    );
}
