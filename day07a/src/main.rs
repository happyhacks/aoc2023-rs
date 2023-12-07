use std::cmp::PartialOrd;
use std::collections::HashSet;
use std::{error, fmt};

use counter::Counter;
use itertools::Itertools;
use strum_macros::EnumIter;

// https://gitlab.com/pezcore/rust-cards/-/tree/master/src
#[derive(EnumIter, Debug, PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
pub enum Rank {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParseError {
    InvalidChar,
    TooShort,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::TooShort => write!(f, "Input string too short"),
            Self::InvalidChar => write!(f, "Invalid character encountered"),
        }
    }
}

impl error::Error for ParseError {}

type Result<T> = std::result::Result<T, ParseError>;

impl TryFrom<char> for Rank {
    type Error = ParseError;
    fn try_from(c: char) -> Result<Self> {
        match c {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::Ten),
            'J' => Ok(Self::Jack),
            'Q' => Ok(Self::Queen),
            'K' => Ok(Self::King),
            'A' => Ok(Self::Ace),
            _ => Err(ParseError::InvalidChar),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
enum Combo {
    HighCard = 1,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct Hand {
    combo: Combo,
    ranks: Vec<Rank>,
    bid: usize,
}

impl Hand {
    fn new(ranks: Vec<Rank>, bid: usize) -> Hand {
        Hand {
            combo: Hand::combo(ranks.clone()),
            ranks,
            bid,
        }
    }
    fn combo(ranks: Vec<Rank>) -> Combo {
        let counter = ranks.into_iter().collect::<Counter<Rank>>();
        let counts: HashSet<usize> = HashSet::from_iter(counter.values().cloned());
        match counter.len() {
            5 => Combo::HighCard,
            4 => Combo::OnePair,
            1 => Combo::FiveOfAKind,
            3 => {
                if counts.contains(&3) {
                    Combo::ThreeOfAKind
                } else {
                    Combo::TwoPair
                }
            }
            2 => {
                if counts.contains(&3) {
                    Combo::FullHouse
                } else {
                    Combo::FourOfAKind
                }
            }
            _ => Combo::HighCard,
        }
    }
}

fn main() {
    let hands: Vec<Hand> = include_str!("/tmp/input.txt")
        .lines()
        .map(|l| {
            let p: Vec<&str> = l.split_ascii_whitespace().collect();
            let cards: Vec<Rank> = p[0].chars().map(|c| Rank::try_from(c).unwrap()).collect();
            Hand::new(cards, p[1].parse().unwrap())
        })
        .sorted()
        .collect();
    println!(
        "{}",
        hands
            .iter()
            .enumerate()
            .map(|(idx, h)| (idx + 1) * h.bid)
            .sum::<usize>()
    );
}
