use itertools::Itertools;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct MapEntry {
    src_start: u32,
    src_end: u32,
    dst_start: u32,
}

macro_rules! range_ok {
    ( $( $name:ident )+ ) => {
        ($($name.0)+) < ($($name.1)+)
    }
}

fn convert(x: (u32, u32), map: HashSet<MapEntry>) -> HashSet<(u32, u32)> {
    let mut done = HashSet::new();
    let mut rest: HashSet<(u32, u32)> = HashSet::from_iter(vec![x]);
    map.iter().for_each(|e| {
        let mut curr_rest = HashSet::new();
        for (start, end) in rest.clone() {
            let ends = vec![(start, end.min(e.src_start)), ((e.src_end).max(start), end)];
            ends.iter()
                .filter(|&range| range_ok!(range))
                .for_each(|&range| {
                    curr_rest.insert(range);
                });
            let mid = (start.max(e.src_start), (e.src_end).min(end));
            if range_ok!(mid) {
                done.insert((
                    mid.0 - (e.src_start - e.dst_start),
                    mid.1 - (e.src_start - e.dst_start),
                ));
            }
        }
        rest = curr_rest;
    });
    done.extend(rest);
    done
}

fn main() {
    let input = include_str!("/tmp/input.txt").trim_end();
    let seeds: Vec<(u32, u32)> = input
        .lines()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .tuples()
        .map(|(a, b)| (a, a + b))
        .collect();
    let stages: Vec<HashSet<MapEntry>> = input
        .split("\n\n")
        .skip(1)
        .map(|block| {
            HashSet::from_iter(block.split('\n').skip(1).map(|line| {
                let val: Vec<u32> = line
                    .split_ascii_whitespace()
                    .map(|x| x.parse().unwrap())
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
            .flat_map(|&seed| {
                stages.iter().fold(
                    HashSet::from_iter(vec![seed]),
                    |acc: HashSet<(u32, u32)>, stage| {
                        acc.iter()
                            .flat_map(|&seed| convert(seed, stage.clone()))
                            .collect()
                    },
                )
            })
            .min()
            .unwrap().0
    );
}
