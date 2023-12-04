use std::collections::HashSet;

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
        match vec![Pos(x + 1, y)]
            .into_iter()
            .filter(|&p| p.is_valid(ln))
            .collect::<Vec<Pos>>()[..]
        {
            [nxt] => Some(nxt),
            _ => None,
        }
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
    fn sym(&self, p: Pos) -> bool {
        let c = self.at(p);
        !c.is_ascii_digit() && c != b'.'
    }
}
fn main() {
    let grid: Vec<Vec<u8>> = include_str!("/tmp/input.txt")
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect();
    let ln = grid.len();
    let g = Grid(grid);
    let mut s: u32 = 0;
    let mut seen = HashSet::new();
    for y in 0..ln {
        for x in 0..ln {
            let mut curr = Pos(x as i32, y as i32);
            let mut neigh = HashSet::new();
            let mut n: u32 = 0;
            if seen.contains(&curr) {
                continue;
            }
            while g.num(curr) {
                seen.insert(curr);
                curr.successors(ln)
                    .iter()
                    .filter(|&&p| g.sym(p))
                    .for_each(|&p| {
                        neigh.insert(p);
                    });
                n = n * 10 + (g.at(curr) - b'0') as u32;
                match curr.next(ln) {
                    Some(nxt) => curr = nxt,
                    None => break,
                }
            }

            if n != 0 && neigh.iter().any(|&p| g.sym(p)) {
                s += n;
            }
        }
    }
    println!("{}", s);
}
