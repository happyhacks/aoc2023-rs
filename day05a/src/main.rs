use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct MapEntry {
    src_start: u32,
    src_end: u32,
    dst_start: u32,
}

fn convert(x: u32, map: HashSet<MapEntry>) -> u32 {
    let mut y = x;
    for e in map {
        if e.src_start <= y && e.src_end > y {
            y -= e.src_start - e.dst_start;
            break;
        }
    }
    y
}

fn main() {
    let input = include_str!("/tmp/input.txt").trim_end();
    let seeds: Vec<u32> = input
        .lines()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let stages: Vec<HashSet<MapEntry>> = input
        .split("\n\n")
        .skip(1)
        .map(|block| {
            HashSet::from_iter(block.split('\n').skip(1).map(|line| {
                let val: Vec<u32> = line
                    .split_ascii_whitespace()
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect();
                MapEntry {
                    src_start: val[1],
                    src_end: val[1] + val[2],
                    dst_start: val[0],
                }
            }))
        })
        .collect();
    println!(
        "{}",
        seeds
            .iter()
            .map(|&seed| {
                stages
                    .iter()
                    .fold(seed, |acc, stage| convert(acc, stage.clone()))
            })
            .min()
            .unwrap()
    );
}
