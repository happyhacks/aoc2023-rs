use std::collections::HashSet;

use ndarray::prelude::*;
use ndarray_linalg::Solve;

#[derive(Clone, Copy, Debug)]
enum Dir {
    North,
    East,
    West,
    South,
}
impl Dir {
    fn all() -> Vec<Dir> {
        vec![Dir::North, Dir::East, Dir::West, Dir::South]
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    // fn is_valid(&self, yln: usize, xln: usize) -> bool {
    //     let p = self;
    //     p.0 >= 0 && p.1 >= 0 && p.0 < xln as i32 && p.1 < yln as i32
    // }
    fn north(&self) -> Pos {
        let &Pos(x, y) = self;
        Pos(x, y - 1)
    }
    fn south(&self) -> Pos {
        let &Pos(x, y) = self;
        Pos(x, y + 1)
    }
    fn east(&self) -> Pos {
        let &Pos(x, y) = self;
        Pos(x + 1, y)
    }
    fn west(&self) -> Pos {
        let &Pos(x, y) = self;
        Pos(x - 1, y)
    }
    fn round(&self, yln: usize, xln: usize) -> Pos {
        let Pos(x, y) = self;
        Pos(x.rem_euclid(xln as i32), y.rem_euclid(yln as i32))
    }
}

struct Grid(Vec<Vec<u8>>);
impl Grid {
    fn yln(&self) -> usize {
        self.0.len()
    }
    fn xln(&self) -> usize {
        if self.yln() > 0 {
            return self.0[0].len();
        }
        0
    }
    fn at(&self, p: Pos) -> u8 {
        let p = p.round(self.yln(), self.xln());
        self.0[p.1 as usize][p.0 as usize]
    }
    fn garden(&self, p: Pos) -> bool {
        self.at(p) != b'#'
    }
    fn get(&self, p: Pos, d: Dir) -> Option<Pos> {
        let next = match d {
            Dir::East => p.east(),
            Dir::West => p.west(),
            Dir::North => p.north(),
            Dir::South => p.south(),
        };
        Some(next)
    }
    fn neigh(&self, p: Pos) -> Vec<Pos> {
        Dir::all()
            .iter()
            .filter_map(|&d| {
                self.get(p, d).and_then(|n| match self.garden(n) {
                    true => Some(n),
                    false => None,
                })
            })
            .collect()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Dist(Pos, usize);

fn count(g: &Grid, n: usize, with_round: bool) -> usize {
    let mut queue = HashSet::new();
    for y in 0..g.yln() {
        for x in 0..g.xln() {
            let curr = Pos(x as i32, y as i32);
            if g.at(curr) == b'S' {
                queue.insert(curr);
            }
        }
    }
    let mut next = HashSet::new();
    for _ in 0..n {
        for p in queue.drain() {
            for n in g.neigh(p) {
                next.insert(n);
            }
        }
        queue = next.clone();
        next.clear();
    }
    if with_round {
        queue
            .iter()
            .map(|p| p.round(g.yln(), g.xln()))
            .collect::<HashSet<Pos>>()
            .len()
    } else {
        queue.len()
    }
}

fn main() {
    let grid: Vec<Vec<u8>> = include_str!("/tmp/input.txt")
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect();
    let g = Grid(grid);
    assert!(g.xln() == g.yln());
    let n = 26501365;
    let b = Array::from(
        (0..3)
            .map(|i| count(&g, (i * g.xln()) + n % g.xln(), false) as f64)
            .collect::<Vec<f64>>(),
    );
    // dbg!(&b); // [3797, 34009, 94353]
    let a: Array2<f64> = array![[0., 0., 1.], [1., 1., 1.], [4., 2., 1.]];
    let x = a.solve_into(b).unwrap(); // [15066, 15146, 3797]
    assert!(x[0] as usize == count(&g, 131, true));
    let p = (n / g.xln()) as f64;
    let y = array![p * p, p, 1.];
    println!("{}", x.dot(&y));
}
