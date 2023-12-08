use std::collections::HashMap;

use itertools::Itertools;

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
    let mut curr = "AAA".to_owned();
    let mut c = 0;
    for mov in directions.chars().cycle() {
        if curr == "ZZZ".to_owned() {
            break;
        }
        c += 1;
        let nxt = network.get(&curr).unwrap().clone();
        curr = match mov {
            'L' => nxt.0,
            'R' => nxt.1,
            _ => unreachable!(),
        }
    }
    println!("{}", c);
}
