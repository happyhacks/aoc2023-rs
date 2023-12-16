use std::{
    collections::{HashSet, VecDeque},
    fmt::{self, Debug},
};
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
    MirrorRight,
    MirrorLeft,
    SplitterVertical,
    SplitterHorizontal,
    #[default]
    None,
}

impl Cell {
    fn is_mirror(&self) -> bool {
        matches!(self, Self::MirrorLeft | Self::MirrorRight)
    }
    fn is_splitter(&self) -> bool {
        matches!(self, Self::SplitterHorizontal | Self::SplitterVertical)
    }
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
            '|' => Ok(Self::SplitterVertical),
            '-' => Ok(Self::SplitterHorizontal),
            '\\' => Ok(Self::MirrorRight),
            '/' => Ok(Self::MirrorLeft),
            _ => Err(ParseError::InvalidChar),
        }
    }
}

impl TryInto<char> for Cell {
    type Error = ParseError;
    fn try_into(self) -> Result<char, ParseError> {
        match self {
            Self::None => Ok('.'),
            Self::SplitterVertical => Ok('|'),
            Self::SplitterHorizontal => Ok('-'),
            Self::MirrorRight => Ok('\\'),
            Self::MirrorLeft => Ok('/'),
        }
    }
}

#[derive(Clone)]
struct Pattern {
    xln: usize,
    yln: usize,
    rows: Vec<Vec<Cell>>,
}

impl Debug for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n")?;
        for y in 0..self.yln {
            for x in 0..self.xln {
                let p = Pos(x as i32, y as i32);
                write!(f, "{}", TryInto::<char>::try_into(self.at(p)).unwrap())?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Pattern {
    fn at(&self, p: Pos) -> Cell {
        assert!(p.is_valid(self.yln, self.xln));
        self.rows[p.1 as usize][p.0 as usize]
    }
    fn from_rows(rows: Vec<Vec<Cell>>) -> Pattern {
        Pattern {
            yln: rows.len(),
            xln: rows[0].len(),
            rows: rows,
        }
    }
    fn get(&self, p: Pos, d: Dir) -> Option<Pos> {
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Beam {
    pos: Pos,
    dir: Dir,
}

impl Beam {
    fn reflect(&mut self, c: Cell) {
        assert!(c.is_mirror());
        self.dir = match c {
            Cell::MirrorRight => match self.dir {
                Dir::North => Dir::West,
                Dir::South => Dir::East,
                Dir::East => Dir::South,
                Dir::West => Dir::North,
            },
            Cell::MirrorLeft => match self.dir {
                Dir::North => Dir::East,
                Dir::South => Dir::West,
                Dir::East => Dir::North,
                Dir::West => Dir::South,
            },
            _ => self.dir,
        }
    }
    fn split(&mut self, c: Cell) -> Option<Beam> {
        assert!(c.is_splitter());
        match c {
            Cell::SplitterVertical => match self.dir {
                Dir::North | Dir::South => None,
                Dir::East | Dir::West => {
                    self.dir = Dir::North;
                    Some(Beam {
                        pos: self.pos,
                        dir: Dir::South,
                    })
                }
            },
            Cell::SplitterHorizontal => match self.dir {
                Dir::North | Dir::South => {
                    self.dir = Dir::East;
                    Some(Beam {
                        pos: self.pos,
                        dir: Dir::West,
                    })
                }
                Dir::East | Dir::West => None,
            },
            _ => None,
        }
    }
}

fn get_count_from(pat: &Pattern, beam: Beam) -> usize {
    let mut visited: HashSet<Beam> = HashSet::new();
    let mut q: VecDeque<Beam> = VecDeque::new();
    q.push_back(beam);
    while let Some(mut curr) = q.pop_front() {
        let cp = curr.pos;
        let cc = pat.at(cp);
        visited.insert(curr);
        if cc.is_mirror() {
            curr.reflect(cc)
        } else if cc.is_splitter() {
            if let Some(next) = curr.split(cc) {
                if !visited.contains(&next) {
                    q.push_back(next);
                }
            }
        }
        match pat.get(curr.pos, curr.dir) {
            Some(np) => {
                let neigh = Beam {
                    pos: np,
                    dir: curr.dir,
                };
                if !visited.contains(&neigh) {
                    q.push_back(neigh);
                }
            }
            None => {}
        }
    }
    return visited
        .iter()
        .map(|x| x.pos)
        .collect::<HashSet<Pos>>()
        .len();
}

fn main() {
    let pat: Pattern = Pattern::from_rows(
        include_str!("/tmp/input.txt")
            .trim_end()
            .split_ascii_whitespace()
            .map(|s| s.chars().map(|c| Cell::try_from(c).unwrap()).collect())
            .collect(),
    );
    println!(
        "{}",
        (0..pat.xln)
            .map(|idx| {
                get_count_from(
                    &pat,
                    Beam {
                        pos: Pos(idx as i32, 0),
                        dir: Dir::South,
                    },
                )
                .max(get_count_from(
                    &pat,
                    Beam {
                        pos: Pos(idx as i32, pat.yln as i32 - 1),
                        dir: Dir::North,
                    },
                ))
            })
            .max()
            .unwrap()
            .max(
                (0..pat.yln)
                    .map(|idx| {
                        get_count_from(
                            &pat,
                            Beam {
                                pos: Pos(0, idx as i32),
                                dir: Dir::East,
                            },
                        )
                        .max(get_count_from(
                            &pat,
                            Beam {
                                pos: Pos(pat.xln as i32 - 1, idx as i32),
                                dir: Dir::West,
                            },
                        ))
                    })
                    .max()
                    .unwrap()
            )
    );
}
