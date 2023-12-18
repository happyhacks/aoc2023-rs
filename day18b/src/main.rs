use std::fmt;

use itertools::Itertools;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{digit1, hex_digit1, one_of};
use nom::combinator::map;
use nom::{sequence::tuple, IResult};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Dir {
    North,
    East,
    West,
    South,
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
impl TryFrom<char> for Dir {
    type Error = ParseError;
    fn try_from(c: char) -> Result<Self, ParseError> {
        match c {
            '3' => Ok(Self::North),
            '1' => Ok(Self::South),
            '2' => Ok(Self::West),
            '0' => Ok(Self::East),
            _ => Err(ParseError::InvalidChar),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Dig {
    direction: Dir,
    count: isize,
}

impl Dig {
    fn from_str(s: &str) -> IResult<&str, Self> {
        let mut parser = tuple((
            one_of("UDLR"),
            tag(" "),
            digit1,
            tag(" (#"),
            take(5usize),
            map(hex_digit1, |s: &str| s.chars().last().unwrap()),
            tag(")"),
        ));

        let (input, (_, _, _, _, count, direction, _)) = parser(s)?;
        Ok((
            input,
            Self {
                direction: Dir::try_from(direction).unwrap(),
                count: isize::from_str_radix(count, 16).unwrap(),
            },
        ))
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(isize, isize);

#[derive(Debug, PartialEq, Clone)]
struct Polygon {
    points: Vec<Pos>,
}

impl Polygon {
    fn from_digs(digs: Vec<Dig>) -> Polygon {
        let points = digs
            .iter()
            .fold((vec![Pos(0, 0)], Pos(0, 0)), |(mut pts, pt), d| {
                let p = match d.direction {
                    Dir::North => Pos(pt.0, pt.1 - d.count),
                    Dir::East => Pos(pt.0 + d.count, pt.1),
                    Dir::West => Pos(pt.0 - d.count, pt.1),
                    Dir::South => Pos(pt.0, pt.1 + d.count),
                };
                pts.push(p);
                (pts, p)
            })
            .0;
        assert!(points.first() == points.last());
        Polygon { points }
    }
    fn shoelace(&self) -> isize {
        1 + self
            .points
            .iter()
            .tuple_windows()
            .map(|(p1, p2)| -> isize {
                (p1.0 * p2.1) - (p1.1 * p2.0)
                    + p1.0.abs_diff(p2.0) as isize
                    + p1.1.abs_diff(p2.1) as isize
            })
            .sum::<isize>()
            / 2
    }
}

fn main() {
    let digs = include_str!("/tmp/input.txt")
        .lines()
        .map(|l| Dig::from_str(l).unwrap().1)
        .collect::<Vec<Dig>>();
    let poly = Polygon::from_digs(digs);
    println!("{}", poly.shoelace());
}
