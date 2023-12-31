use std::{
    collections::HashSet,
    fmt::{self, Debug},
};
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
enum Dir {
    North,
    East,
    West,
    South,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn is_valid(&self, yln: usize, xln: usize) -> bool {
        let p = self;
        p.0 >= 0 && p.1 >= 0 && p.0 < xln as i32 && p.1 < yln as i32
    }
    fn north(&self) -> Pos {
        let &Pos(x, y) = self;
        Pos(x, y - 1)
    }
    fn south(&self) -> Pos {
        let &Pos(x, y) = self;
        Pos(x, y + 1)
    }
    fn east(&self) -> Pos {
        let &Pos(x, y) = self;
        Pos(x + 1, y)
    }
    fn west(&self) -> Pos {
        let &Pos(x, y) = self;
        Pos(x - 1, y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
enum Cell {
    Round,
    Cube,
    #[default]
    None,
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
impl TryFrom<char> for Cell {
    type Error = ParseError;
    fn try_from(c: char) -> Result<Self, ParseError> {
        match c {
            '.' => Ok(Self::None),
            '#' => Ok(Self::Cube),
            'O' => Ok(Self::Round),
            _ => Err(ParseError::InvalidChar),
        }
    }
}

#[derive(Clone)]
struct Pattern {
    xln: usize,
    yln: usize,
    round: Vec<Pos>,
    cube: HashSet<Pos>,
}

impl Debug for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n")?;
        for y in 0..self.yln {
            for x in 0..self.xln {
                let p = Pos(x as i32, y as i32);
                if self.cube.contains(&p) {
                    write!(f, "#")?;
                } else if self.round.contains(&p) {
                    write!(f, "O")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Pattern {
    fn from_rows(rows: Vec<Vec<Cell>>) -> Pattern {
        let mut p = Pattern {
            yln: rows.len(),
            xln: rows[0].len(),
            round: Vec::new(),
            cube: HashSet::new(),
        };
        rows.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, cell)| match cell {
                Cell::Round => p.round.push(Pos(x as i32, y as i32)),
                Cell::Cube => {
                    p.cube.insert(Pos(x as i32, y as i32));
                }
                _ => {}
            })
        });
        p
    }
    fn roll(&self, p: Pos, d: Dir) -> Option<Pos> {
        let next = match d {
            Dir::East => p.east(),
            Dir::West => p.west(),
            Dir::North => p.north(),
            Dir::South => p.south(),
        };
        if next.is_valid(self.yln, self.xln) {
            return Some(next);
        }
        None
    }
    fn order(&mut self, d: Dir) {
        self.round.sort_by(|a, b| match d {
            Dir::East => b.0.cmp(&a.0),
            Dir::West => a.0.cmp(&b.0),
            Dir::North => b.1.cmp(&a.1),
            Dir::South => a.1.cmp(&b.1),
        })
    }
    fn tilt(&mut self, d: Dir) {
        self.order(d);
        let count = match d {
            Dir::East | Dir::West => self.xln,
            Dir::North | Dir::South => self.yln,
        };
        for _ in 0..count {
            self.round = self
                .round
                .iter()
                .map(|&p| match self.roll(p, d) {
                    Some(nxt) => {
                        if self.cube.contains(&nxt) || self.round.contains(&nxt) {
                            p
                        } else {
                            nxt
                        }
                    }
                    None => p,
                })
                .collect();
        }
    }
    fn score(&self) -> usize {
        self.round.iter().map(|Pos(_, y)| self.yln as i32-y).sum::<i32>() as usize
    }
}

fn main() {
    let mut pat: Pattern = Pattern::from_rows(
        include_str!("/tmp/input.txt")
            .trim_end()
            .split_ascii_whitespace()
            .map(|s| s.chars().map(|c| Cell::try_from(c).unwrap()).collect())
            .collect(),
    );
    pat.tilt(Dir::North);
    println!("{}", pat.score());
}
