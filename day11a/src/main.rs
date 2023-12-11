use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize, usize);

struct Grid {
    xln: usize,
    yln: usize,
    points: Vec<Pos>,
    empty_row: HashSet<usize>,
    empty_col: HashSet<usize>,
}
impl Grid {
    fn new_fromvec(g: Vec<Vec<u8>>, points: Vec<Pos>) -> Self {
        let mut g = Grid {
            yln: g.len(),
            xln: g[0].len(),
            points: points,
            empty_row: HashSet::new(),
            empty_col: HashSet::new(),
        };
        g.set_empty();
        g
    }
    fn set_empty(&mut self) {
        let (all_x, all_y) =
            self.points
                .iter()
                .fold((HashSet::new(), HashSet::new()), |(mut x, mut y), p| {
                    x.insert(p.0);
                    y.insert(p.1);
                    (x, y)
                });
        self.empty_row
            .extend((0..self.xln).filter(|x| !all_x.contains(x)));
        self.empty_col
            .extend((0..self.yln).filter(|x| !all_y.contains(x)));
    }
    fn distance(&self, a: Pos, b: Pos) -> usize {
        usize::abs_diff(a.0, b.0)
            + 1 * self.empty_x_between(a.0, b.0)
            + usize::abs_diff(a.1, b.1)
            + 1 * self.empty_y_between(a.1, b.1)
    }
    fn empty_x_between(&self, a: usize, b: usize) -> usize {
        (a.min(b)..b.max(a))
            .filter(|x| self.empty_row.contains(x))
            .count()
    }
    fn empty_y_between(&self, a: usize, b: usize) -> usize {
        (a.min(b)..b.max(a))
            .filter(|x| self.empty_col.contains(x))
            .count()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Dist(Pos, usize);

fn main() {
    let g: Vec<Vec<u8>> = include_str!("/tmp/input.txt")
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect();
    let mut points = Vec::new();
    for (y, row) in g.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == b'#' {
                points.push(Pos(x, y, points.len() + 1))
            }
        }
    }
    let g = Grid::new_fromvec(g, points);
    println!(
        "{}",
        g.points
            .iter()
            .map(|&a| {
                g.points
                    .iter()
                    .map(|&b| {
                        let d = g.distance(a, b);
                        d
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
            / 2
    );
}
