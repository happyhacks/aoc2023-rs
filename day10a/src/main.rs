use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

#[derive(Clone, Copy, Debug)]
enum Dir {
    North,
    East,
    West,
    South,
}
impl Dir {
    fn invert(&self) -> Dir {
        match self {
            Dir::East => Dir::West,
            Dir::West => Dir::East,
            Dir::North => Dir::South,
            Dir::South => Dir::North,
        }
    }
    fn all() -> Vec<Dir> {
        vec![Dir::North, Dir::East, Dir::West, Dir::South]
    }
}
fn opens(b: u8, d: Dir) -> bool {
    match (b, d) {
        (b'S', _) => true,
        (b'|', Dir::South) => true,
        (b'|', Dir::North) => true,
        (b'-', Dir::East) => true,
        (b'-', Dir::West) => true,
        (b'F', Dir::East) => true,
        (b'F', Dir::South) => true,
        (b'L', Dir::North) => true,
        (b'L', Dir::East) => true,
        (b'J', Dir::West) => true,
        (b'J', Dir::North) => true,
        (b'7', Dir::West) => true,
        (b'7', Dir::South) => true,
        _ => false,
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn is_valid(&self, ln: usize) -> bool {
        let p = self;
        p.0 >= 0 && p.1 >= 0 && p.0 < ln as i32 && p.1 < ln as i32
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
    fn ln(&self) -> usize {
        self.0.len()
    }
    fn at(&self, p: Pos) -> u8 {
        self.0[p.1 as usize][p.0 as usize]
    }
    fn empty(&self, p: Pos) -> bool {
        self.at(p) == b'.'
    }
    fn get(&self, p: Pos, d: Dir) -> Option<Pos> {
        let next = match d {
            Dir::East => p.east(),
            Dir::West => p.west(),
            Dir::North => p.north(),
            Dir::South => p.south(),
        };
        if next.is_valid(self.ln()) {
            return Some(next);
        }
        None
    }
    fn neigh(&self, p: Pos) -> Vec<Pos> {
        Dir::all()
            .iter()
            .filter(|&&d| opens(self.at(p), d))
            .filter_map(|&d| {
                self.get(p, d)
                    .and_then(|n| match opens(self.at(n), d.invert()) {
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
    let ln = grid.len();
    let g = Grid(grid);
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    for y in 0..ln {
        for x in 0..ln {
            let curr = Pos(x as i32, y as i32);
            if g.at(curr) == b'S' {
                visited.insert(curr);
                queue.push_back(Dist(curr, 0))
            }
        }
    }
    while let Some(Dist(p, d)) = queue.pop_front() {
        if !g.empty(p) && g.neigh(p).iter().all(|n| visited.contains(n)) {
            println!("{}", d + 1);
            break;
        }
        for n in g.neigh(p) {
            if !visited.contains(&n) {
                visited.insert(n);
                queue.push_back(Dist(n, d + 1))
            }
        }
    }
}
