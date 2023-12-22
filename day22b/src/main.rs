use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    vec,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Pos<T>
where
    T: Clone + Copy,
{
    fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
    fn from_vec(v: Vec<T>) -> Self {
        assert!(v.len() == 3);
        Self {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }
}

impl<T> std::ops::Sub for Pos<T>
where
    T: std::ops::Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> Debug for Pos<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.x, self.y, self.z)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Plane {
    X,
    Y,
    Z,
    Point,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Brick<T>
where
    T: Display,
{
    start: Pos<T>,
    end: Pos<T>,
    plane: Plane,
}

impl<T> Brick<T>
where
    T: Default
        + Display
        + Clone
        + Copy
        + PartialOrd
        + PartialEq
        + Debug
        + std::ops::Sub<Output = T>,
{
    fn new(start: Pos<T>, end: Pos<T>) -> Self {
        assert!(
            start.x == end.x || start.y == end.y || start.z == end.z,
            "{start:?} :{end:?} is plane"
        );
        let plane = if start.x != end.x {
            Plane::X
        } else if start.y != end.y {
            Plane::Y
        } else if start.z != end.z {
            Plane::Z
        } else if start.x == end.x && start.y == end.y && start.z == end.z {
            Plane::Point
        } else {
            unreachable!()
        };
        Self { start, end, plane }
    }
}

impl<T> Debug for Brick<T>
where
    T: Debug + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} -> {:?} | {:?}", self.start, self.end, self.plane)
    }
}

fn range_from(a: i16, b: i16) -> std::ops::Range<i16> {
    if a < b {
        a..(b + 1)
    } else {
        b..(a + 1)
    }
}

impl Brick<i16> {
    fn points(&self) -> HashSet<Pos<i16>> {
        match self.plane {
            Plane::X => range_from(self.start.x, self.end.x)
                .map(|x| Pos::new(x, self.start.y, self.start.z))
                .collect(),
            Plane::Y => range_from(self.start.y, self.end.y)
                .map(|y| Pos::new(self.start.x, y, self.start.z))
                .collect(),
            Plane::Z => range_from(self.start.z, self.end.z)
                .map(|z| Pos::new(self.start.x, self.start.y, z))
                .collect(),
            Plane::Point => vec![self.start].into_iter().collect(),
        }
    }
    fn intersects(&self, other: &Self) -> bool {
        // dbg!(&self, &other);
        // self.points().intersection(&other.points()).count() > 0
        !self.points().is_disjoint(&other.points())
    }
    fn down(&mut self) -> bool {
        if self.start.z <= 1 || self.end.z <= 1 {
            return false;
        }
        self.start.z -= 1;
        self.end.z -= 1;
        true
    }
    fn up(&mut self) {
        self.start.z += 1;
        self.end.z += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersects_common_end_points() {
        let b1 = Brick::new(Pos::new(0, 0, 2), Pos::new(2, 0, 2));
        let b2 = Brick::new(Pos::new(0, 0, 2), Pos::new(0, 2, 2));
        assert!(b1.intersects(&b2));
    }

    #[test]
    fn intersects_point_on_line() {
        let b1 = Brick::new(Pos::new(0, 0, 2), Pos::new(0, 0, 0));
        let b2 = Brick::new(Pos::new(0, 0, 1), Pos::new(0, 0, 1));
        assert!(b1.intersects(&b2));
    }

    #[test]
    fn intersects_cross() {
        let b1 = Brick::new(Pos::new(1, 0, 0), Pos::new(1, 2, 0));
        let b2 = Brick::new(Pos::new(0, 1, 0), Pos::new(2, 1, 0));
        assert!(b1.intersects(&b2));
    }
}

fn main() {
    let mut bricks = include_str!("/tmp/input.txt")
        .trim_end()
        .lines()
        .map(|l| {
            let mut parts = l.split("~");
            let t = |p: &str| {
                let v = p.split(",").map(|s| s.parse::<i16>().unwrap()).collect();
                Pos::from_vec(v)
            };
            Brick::new(t(parts.next().unwrap()), t(parts.next().unwrap()))
        })
        .collect::<Vec<_>>();
    bricks.sort_by_key(|b| b.start.z.min(b.end.z) as i16);
    for i in 0..bricks.len() {
        let mut bad_move = false;
        while bricks[i].down() {
            if !bricks
                .iter()
                .enumerate()
                .all(|(idx, &b2)| !(i != idx && bricks[i].intersects(&b2)))
            {
                bad_move = true;
                break;
            }
        }
        if bad_move {
            bricks[i].up();
        }
    }
    bricks.sort_by_key(|b| b.start.z.min(b.end.z) as i16);
    assert!(!bricks
        .iter()
        .any(|b| bricks.iter().any(|b2| b != b2 && b.intersects(b2))));
    let mut c = 0;
    let bricks = bricks;
    for i in 0..bricks.len() {
        let mut curr: Vec<Brick<i16>> = bricks
            .clone()
            .iter()
            .enumerate()
            .filter_map(|(idx, &b)| if i == idx { None } else { Some(b) })
            .collect();
        for i in 0..curr.len() {
            let mut bad_move = false;
            let mut good_move = false;
            while curr[i].down() {
                if !curr
                    .iter()
                    .enumerate()
                    .all(|(idx, &b2)| !(i != idx && curr[i].intersects(&b2)))
                {
                    bad_move = true;
                    break;
                } else {
                    good_move = true;
                }
            }
            if bad_move {
                curr[i].up();
            }
            if good_move {
                c += 1;
            }
        }
    }
    println!("{}", c);
}
