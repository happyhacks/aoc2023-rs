use std::collections::HashSet;

fn main() {
    println!(
        "{}",
        include_str!("/tmp/input.txt")
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
            .filter(|&p| p > 0)
            .map(|p| 1 << (p - 1))
            .sum::<usize>()
    )
}
