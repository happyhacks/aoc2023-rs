use std::collections::HashSet;

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
    fn is_valid(&self, yln: usize, xln: usize) -> bool {
        let p = self;
        p.0 >= 0 && p.1 >= 0 && p.0 < xln as i32 && p.1 < yln as i32
    }
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
        if next.is_valid(self.yln(), self.xln()) {
            return Some(next);
        }
        None
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

fn main() {
    let grid: Vec<Vec<u8>> = include_str!("/tmp/input.txt")
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect();
    let g = Grid(grid);
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
    for _ in 0..64 {
        for p in queue.drain() {
            for n in g.neigh(p) {
                next.insert(n);
            }
        }
        queue = next.clone();
        next.clear();
    }
    println!("{}", queue.len());
    for y in 0..g.yln() {
        for x in 0..g.xln() {
            let curr = Pos(x as i32, y as i32);
            if queue.contains(&curr) {
                print!("O");
            } else {
                print!("{}", g.at(curr) as char);
            }
        }
        println!();
    }
}
