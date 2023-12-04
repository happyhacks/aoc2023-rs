use std::collections::HashSet;

fn main() {
    let wins: Vec<usize> = include_str!("/tmp/input.txt")
        .lines()
        .map(|l| {
            let cards: Vec<HashSet<u32>> = l
                .split(':')
                .nth(1)
                .unwrap()
                .split('|')
                .map(|c| {
                    HashSet::from_iter(
                        c.split_ascii_whitespace()
                            .map(|c| c.parse::<u32>().unwrap()),
                    )
                })
                .collect();
            cards[0].intersection(&cards[1]).count()
        })
        .collect();
    let mut cards = vec![1; wins.len()];
    for (idx, card) in wins.into_iter().enumerate() {
        for i in 1usize..=card {
            if idx+i < cards.len() {
                cards[idx+i] += cards[idx];
            }
        }
    }
    println!("{}", cards.iter().sum::<u32>());

}
