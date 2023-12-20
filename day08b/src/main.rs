use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use petgraph::{
    dot::{Config, Dot},
    graphmap::DiGraphMap,
};

fn main() {
    let input = include_str!("/tmp/input.txt").trim_end();
    let directions = input.lines().next().unwrap();
    let network = input
        .split("\n\n")
        .nth(1)
        .unwrap()
        .lines()
        .map(|l| {
            let mut parts = l.split(" = ");
            let src = parts.next().unwrap().to_owned();
            let dst: (String, String) = parts
                .next()
                .unwrap()
                .split(", ")
                .map(|p| p.replace("(", "").replace(")", "").to_owned())
                .collect_tuple()
                .unwrap();
            (src, dst)
        })
        .fold(HashMap::new(), |mut h, (k, v)| {
            h.insert(k, v);
            h
        });
    let g: DiGraphMap<&str, ()> = DiGraphMap::from_edges(
        network
            .iter()
            .flat_map(|(k, v)| vec![(k.as_str(), v.0.as_str()), (k.as_str(), v.1.as_str())])
            .collect::<Vec<(&str, &str)>>(),
    );

    if cfg!(debug_assertions) {
        println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
    }
    assert!(
        HashSet::<String>::from_iter(
            network
                .iter()
                .filter(|(k, _)| k.ends_with("A"))
                .flat_map(|(_, v)| vec![v.0.clone(), v.1.clone()])
        ) == HashSet::<String>::from_iter(
            network
                .iter()
                .filter(|(k, _)| k.ends_with("Z"))
                .flat_map(|(_, v)| vec![v.0.clone(), v.1.clone()])
        )
    );

    let mut currs: Vec<String> = network
        .keys()
        .filter(|&k| k.ends_with("A"))
        .cloned()
        .collect();
    let mut c = 0u64;
    let mut times = vec![1; currs.len()];

    for mov in directions.chars().cycle() {
        currs
            .iter()
            .enumerate()
            .filter(|(_, k)| k.ends_with("Z"))
            .for_each(|(idx, _)| {
                assert!(times[idx] == 1); // never again - single cycle - shortest cycle
                times[idx] = c;
            });
        if times.iter().all(|&t| t > 1) {
            break;
        }
        c += 1;
        currs.iter_mut().for_each(|curr| {
            let nxt = network.get(curr).unwrap().clone();
            *curr = match mov {
                'L' => nxt.0,
                'R' => nxt.1,
                _ => unreachable!(),
            }
        })
    }
    dbg!(times.clone());
    println!(
        "{}",
        times.into_iter().reduce(num::integer::lcm).unwrap()
    );
}
