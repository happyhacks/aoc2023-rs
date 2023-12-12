use std::fmt;

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
}

impl Record {
    fn count(&self, rx: usize, dx: usize, cdmg: usize) -> usize {
        if rx == self.row.len() {
            return match (dx, cdmg) {
                (x, 0) if x == self.damaged.len() => 1, 
                (x, y) if x == self.damaged.len() - 1 && y == self.damaged[x] => 1,
                _ => 0,
            }
        }
        let mut count = 0;
        match self.row[rx] {
            Spring::Damaged=> {
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
        count
    }
}

fn main() {
    let records: Vec<Record> = include_str!("/tmp/input.txt")
        .trim_end()
        .lines()
        .map(|l| {
            let mut parts = l.split_ascii_whitespace();
            Record {
                row: parts
                    .next()
                    .unwrap()
                    .chars()
                    .map(|c| Spring::try_from(c).unwrap())
                    .collect(),
                damaged: parts
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|n| n.parse().unwrap())
                    .collect(),
            }
        })
        .collect();
    // dbg!(records);
    println!(
        "{}",
        records.iter().map(|r| r.count(0, 0, 0)).sum::<usize>()
    );
}
