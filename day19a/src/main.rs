use std::collections::HashMap;
use std::fmt::Debug;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_until, take_while1};
use nom::character::complete::{digit1, one_of};
use nom::combinator::map;
use nom::error::ErrorKind;
use nom::multi::separated_list1;
use nom::{sequence::tuple, IResult};

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
    fn process(&self, r: &Rating) -> String {
        for step in self.steps.iter() {
            if step.terminate {
                return step.target.clone();
            } else {
                let v = match step.operation {
                    Op::LessThan => step.value > r.values[step.category as usize].1,
                    Op::GreaterThan => step.value < r.values[step.category as usize].1,
                    _ => unreachable!(),
                };
                if v {
                    return step.target.clone();
                }
            }
        }
        unreachable!()
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

fn process(w: &HashMap<String, Workflow>, r: Rating) -> bool {
    let mut curr = "in".to_string();
    while let Some(s) = w.get(&curr) {
        curr = s.process(&r);
    }
    // dbg!((&r, &curr));
    curr == "A".to_string()
}

fn main() {
    let (workflows, ratings) = include_str!("/tmp/input.txt")
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
    println!(
        "{}",
        ratings
            .iter()
            .filter_map(|r| {
                if process(&workflows, r.clone()) {
                    return Some(r.values.iter().map(|&x| x.1).sum::<isize>());
                }
                None
            })
            .sum::<isize>()
    );
}
