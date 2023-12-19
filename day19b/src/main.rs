use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Range;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_until, take_while1};
use nom::character::complete::{digit1, one_of};
use nom::combinator::map;
use nom::error::ErrorKind;
use nom::multi::separated_list1;
use nom::{sequence::tuple, IResult};
use pathfinding::directed::count_paths::count_paths;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    X,
    M,
    A,
    S,
    None,
}

impl TryFrom<char> for Category {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'x' => Ok(Category::X),
            'm' => Ok(Category::M),
            'a' => Ok(Category::A),
            's' => Ok(Category::S),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    LessThan,
    GreaterThan,
    None,
}

impl TryFrom<char> for Op {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '<' => Ok(Op::LessThan),
            '>' => Ok(Op::GreaterThan),
            _ => Err(()),
        }
    }
}
#[derive(Debug)]
struct Step {
    terminate: bool,
    category: Category,
    operation: Op,
    value: isize,
    target: String,
}

impl Step {
    fn from_str(s: &str) -> IResult<&str, Self> {
        let mut parser = alt((
            map(
                tuple((
                    one_of::<_, _, (&str, ErrorKind)>("xmas"),
                    one_of("<>"),
                    digit1,
                    tag(":"),
                    take_while1(char::is_alphabetic),
                )),
                |(category, operation, value, _, target)| Self {
                    category: Category::try_from(category).unwrap(),
                    operation: Op::try_from(operation).unwrap(),
                    value: value.parse().unwrap(),
                    target: target.to_string(),
                    terminate: false,
                },
            ),
            map(take_while1(char::is_alphabetic), |target: &str| Self {
                category: Category::None,
                operation: Op::None,
                value: 0,
                target: target.to_string(),
                terminate: true,
            }),
        ));
        let (res, step) = parser(s).unwrap();
        Ok((res, step))
    }
}
#[derive(Debug)]
struct Workflow {
    name: String,
    steps: Vec<Step>,
}

impl Workflow {
    fn from_str(s: &str) -> IResult<&str, Self> {
        let mut parser = tuple((
            take_until("{"),
            tag("{"),
            separated_list1(tag(","), Step::from_str),
            tag("}"),
        ));
        let (res, (name, _, steps, _)) = parser(s)?;
        assert!(steps.last().unwrap().terminate);
        Ok((
            res,
            Self {
                name: name.to_string(),
                steps,
            },
        ))
    }
}
#[derive(Clone)]
struct Rating {
    values: Vec<(Category, isize)>,
}

impl Debug for Rating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for &(category, value) in self.values.iter() {
            write!(f, "{:?}={:?},", category, value)?;
        }
        write!(f, "}}")
    }
}

impl Rating {
    fn from_str(s: &str) -> IResult<&str, Self> {
        let mut parser = tuple((
            tag("{"),
            map(
                separated_list1(
                    tag(","),
                    tuple((
                        map(one_of("xmas"), |c| Category::try_from(c).unwrap()),
                        tag("="),
                        map(digit1, |i: &str| i.parse().unwrap()),
                    )),
                ),
                |l| {
                    l.iter()
                        .map(|&(a, _, b)| (a, b))
                        .collect::<Vec<(Category, isize)>>()
                },
            ),
            tag("}"),
        ));
        let (res, (_, values, _)) = parser(s)?;
        assert!(values.len() == 4);
        let mut values = values.clone();
        values.sort_by_key(|x| x.0 as usize);
        Ok((res, Self { values }))
    }
}

fn main() {
    let (mut workflows, _) = include_str!("/tmp/input.txt")
        .split_once("\n\n")
        .and_then(|(a, b)| {
            Some((
                a.lines()
                    .map(|l| Workflow::from_str(l).unwrap().1)
                    .map(|w| (w.name.clone(), w))
                    .collect::<HashMap<String, Workflow>>(),
                b.lines()
                    .map(|l| Rating::from_str(l).unwrap().1)
                    .collect::<Vec<Rating>>(),
            ))
        })
        .unwrap();
    workflows.insert(
        "R".to_string(),
        Workflow {
            name: "R".to_string(),
            steps: vec![],
        },
    );
    let successors = |(n, cons): &(String, Vec<Range<isize>>)| {
        let w = workflows
            .get(n)
            .unwrap_or_else(|| panic!("{} not found", n));
        let mut neigh = Vec::new();
        let mut conn = cons.clone();
        for s in w.steps.iter() {
            let mut cont = conn.clone();
            match s.operation {
                Op::LessThan => {
                    cont[s.category as usize] = cont[s.category as usize].start..s.value;
                }
                Op::GreaterThan => {
                    cont[s.category as usize] = s.value + 1..cont[s.category as usize].end;
                }
                Op::None => {}
            }
            neigh.push((s.target.clone(), cont));
            match s.operation {
                Op::GreaterThan => {
                    conn[s.category as usize] = conn[s.category as usize].start..s.value + 1;
                }
                Op::LessThan => {
                    conn[s.category as usize] = s.value..conn[s.category as usize].end;
                }
                Op::None => {}
            }
        }
        neigh
    };
    let mut s: usize = 0;
    let success = |(n, cons): &(String, Vec<Range<isize>>)| {
        *n == "A".to_string() && {
            // dbg!(cons);
            s += cons
                .iter()
                .fold(1usize, |prod, c| prod * (c.end - c.start) as usize);
            true
        }
    };
    count_paths(("in".to_string(), vec![1..4001; 4]), successors, success);
    println!("{}", s);
}
