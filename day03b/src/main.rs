use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn is_valid(&self, ln: usize) -> bool {
        let p = self;
        p.0 >= 0 && p.1 >= 0 && p.0 < ln as i32 && p.1 < ln as i32
    }

    fn successors(&self, ln: usize) -> Vec<Pos> {
        let &Pos(x, y) = self;
        vec![
            Pos(x + 1, y + 1),
            Pos(x, y + 1),
            Pos(x - 1, y + 1),
            Pos(x + 1, y - 1),
            Pos(x, y - 1),
            Pos(x - 1, y - 1),
            Pos(x - 1, y),
            Pos(x + 1, y),
        ]
        .into_iter()
        .filter(|&p| p.is_valid(ln))
        .collect()
    }

    fn next(&self, ln: usize) -> Option<Pos> {
        let &Pos(x, y) = self;
        let nxt = Pos(x + 1, y);
        if nxt.is_valid(ln) {
            return Some(nxt);
        }
        None
    }

    fn prev(&self, ln: usize) -> Option<Pos> {
        let &Pos(x, y) = self;
        let nxt = Pos(x - 1, y);
        if nxt.is_valid(ln) {
            return Some(nxt);
        }
        None
    }
}

struct Grid(Vec<Vec<u8>>);
impl Grid {
    fn at(&self, p: Pos) -> u8 {
        self.0[p.1 as usize][p.0 as usize]
    }
    fn num(&self, p: Pos) -> bool {
        self.at(p).is_ascii_digit()
    }
    fn star(&self, p: Pos) -> bool {
        self.at(p) == b'*'
    }
}
fn _main() {
    let grid: Vec<Vec<u8>> = include_str!("/tmp/input.txt")
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect();
    let ln = grid.len();
    let g = Grid(grid);
    let mut s: u32 = 0;
    for y in 0..ln {
        for x in 0..ln {
            let curr = Pos(x as i32, y as i32);
            let mut n: u32 = 1;
            if !g.star(curr) {
                continue;
            }
            let mut neigh = HashSet::new();
            curr.successors(ln)
                .into_iter()
                .filter(|&p| g.num(p))
                .for_each(|pos| {
                    let mut c = pos;
                    while g.num(c) {
                        match c.prev(ln) {
                            Some(nxt) => c = nxt,
                            None => break,
                        }
                    }
                    if g.num(c) {
                        neigh.insert(c);
                    } else {
                        neigh.insert(c.next(ln).unwrap());
                    }
                });
            if neigh.len() != 2 {
                continue;
            }
            for num in neigh {
                let mut cnum = num;
                let mut cn = 0;
                while g.num(cnum) {
                    cn = cn * 10 + (g.at(cnum) - b'0') as u32;
                    match cnum.next(ln) {
                        Some(nxt) => cnum = nxt,
                        None => break,
                    }
                }
                // print!("{}, ", cn);
                n *= cn;
            }
            // println!("");
            s += n;
        }
    }
    println!("{}", s);
}

fn main() {
    let grid: Vec<Vec<u8>> = include_str!("/tmp/input.txt")
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect();
    let ln = grid.len();
    let g = Grid(grid);
    let mut seen = HashSet::new();
    let mut gears = HashMap::new();
    for y in 0..ln {
        for x in 0..ln {
            let mut curr = Pos(x as i32, y as i32);
            let mut neigh_stars = HashSet::new();
            let mut n: u32 = 0;
            if seen.contains(&curr) {
                continue;
            }
            while g.num(curr) {
                seen.insert(curr);
                curr.successors(ln)
                    .iter()
                    .filter(|&&p| g.star(p))
                    .for_each(|&p| {
                        neigh_stars.insert(p);
                    });
                n = n * 10 + (g.at(curr) - b'0') as u32;
                match curr.next(ln) {
                    Some(nxt) => curr = nxt,
                    None => break,
                }
            }

            if n != 0 {
                neigh_stars.iter().for_each(|&pos| {
                    gears.entry(pos).or_insert(vec![]).push(n);
                })
            }
        }
    }

    println!(
        "{}",
        gears
            .values()
            .filter(|v| v.len() == 2)
            .map(|v| v.iter().fold(1, |acc, x| acc * x))
            .sum::<u32>()
    );
}
