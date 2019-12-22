use std::ops::{Add, AddAssign, Sub, Mul};

pub type Len = u32;
pub type Coord = i32;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Point {
    pub x: Coord,
    pub y: Coord,
}

impl Point {
    pub fn origin() -> Self {
        Self { x: 0, y: 0 }
    }

    #[allow(dead_code)]
    pub fn walk(self, dir: Dir, len: Len) -> impl Iterator<Item = Point> {
        Walker::new(self, dir, len)
    }

    pub fn walk_many<Path>(self, path: Path) -> impl Iterator<Item = Point>
        where Path: IntoIterator<Item = (Dir, Len)>
    {
        WalkMany::new(self, path.into_iter())
    }
}

impl Sub<Point> for Point {
    type Output = Move;
    fn sub(self, other: Self) -> Move {
        Move {
            dx: self.x - other.x,
            dy: self.y - other.y,
        }
    }
}

impl Add<Move> for Point {
    type Output = Self;
    fn add(self, other: Move) -> Self {
        Self {
            x: self.x + other.dx,
            y: self.y + other.dy,
        }
    }
}

impl AddAssign<Move> for Point {
    fn add_assign(&mut self, other: Move) {
        *self = *self + other;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    pub dx: Coord,
    pub dy: Coord,
}

impl Add<Move> for Move {
    type Output = Self;
    fn add(self, other: Move) -> Self {
        Self {
            dx: self.dx + other.dx,
            dy: self.dy + other.dy,
        }
    }
}

impl Mul<Len> for Move {
    type Output = Self;
    fn mul(self, other: Len) -> Self {
        Self {
            dx: self.dx * (other as Coord),
            dy: self.dy * (other as Coord),
        }
    }
}

impl Move {
    pub fn len(self) -> Len {
        (self.dx.abs() as Len) + (self.dy.abs() as Len)
    }

    pub fn turn_left(self) -> Move {
        Move { dx: -self.dy, dy: self.dx }
    }

    pub fn turn_right(self) -> Move {
        Move { dx: self.dy, dy: -self.dx }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Dir {
    Rt,
    Up,
    Lf,
    Dn
}

impl Dir {
    pub fn to_move(self) -> Move {
        match self {
            Dir::Rt => Move { dx: 1, dy: 0 },
            Dir::Up => Move { dx: 0, dy: 1 },
            Dir::Lf => Move { dx: -1, dy: 0 },
            Dir::Dn => Move { dx: 0, dy: -1 },
        }
    }

    pub fn from_char(ch: char) -> Option<Dir> {
        match ch {
            'R' => Some(Dir::Rt),
            'U' => Some(Dir::Up),
            'L' => Some(Dir::Lf),
            'D' => Some(Dir::Dn),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
struct Walker {
    here: Point,
    dir: Move,
    len: Len,
}

impl Iterator for Walker {
    type Item = Point;
    fn next(&mut self) -> Option<Point> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        self.here += self.dir;
        Some(self.here)
    }
}

impl Walker {
    fn new(from: Point, dir: Dir, len: Len) -> Self {
        Self { here: from, dir: dir.to_move(), len }
    }
}

#[derive(Clone, Debug)]
struct WalkMany<Path>
    where Path: Iterator<Item = (Dir, Len)>
{
    cur: Walker,
    rest: Path
}

impl<Path> WalkMany<Path>
    where Path: Iterator<Item = (Dir, Len)>
{
    fn new(start: Point, path: Path) -> Self {
        let mut path = path.into_iter();
        if let Some((dir, len)) = path.next() {
            Self { cur: Walker::new(start, dir, len), rest: path }
        } else {
            // This is annoying.  A dummy length-0 walker almost
            // works, but is incorrect if the iterator isn't "fused".
            // Correcting for that requires an extra check or dispatch,
            // and this case won't be used in practice.
            unimplemented!("empty path in Point::walk_many")
        }
    }
}

impl<Path> Iterator for WalkMany<Path>
    where Path: Iterator<Item = (Dir, Len)>
{
    type Item = Point;
    fn next(&mut self) -> Option<Point> {
        loop {
            if let Some(pt) = self.cur.next() {
                return Some(pt)
            }
            if let Some((dir, len)) = self.rest.next() {
                self.cur = Walker::new(self.cur.here, dir, len)
            } else {
                return None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Point, Dir, Len};
    use ::quickcheck::*;
    use quickcheck_macros::quickcheck;
    use std::collections::HashSet;
    use std::hash::Hash;
    use std::iter;

    impl Arbitrary for Point {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let (x, y) = Arbitrary::arbitrary(g);
            Point { x, y }
        }
    }

    impl Arbitrary for Dir {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let b: usize = Arbitrary::arbitrary(g);
            [Dir::Rt, Dir::Up, Dir::Lf, Dir::Dn][b & 3]
        }
    }

    fn endpt(p: Point, d: Dir, l: Len) -> Point {
        p + d.to_move() * l
    }

    fn is_unique<I>(stuff: I) -> bool
        where I: IntoIterator,
              I::Item: Hash + Eq
    {
        let mut seen = HashSet::new();
        for thing in stuff {
            if !seen.insert(thing) {
                return false;
            }
        }
        return true;
    }

    fn dir_rev(d: Dir) -> Dir {
        match d {
            Dir::Rt => Dir::Lf,
            Dir::Up => Dir::Dn,
            Dir::Lf => Dir::Rt,
            Dir::Dn => Dir::Up,
        }
    }

    fn vec_rev<T>(mut v: Vec<T>) -> Vec<T> {
        v.reverse();
        v
    }

    #[quickcheck]
    fn selftest_unique_true(x: usize, y: usize, z: usize) -> bool {
        is_unique(&[x, y, z]) == (x != y && y != z && x != z)
    }

    #[quickcheck]
    fn selftest_unique_false(x: usize, y: usize, z: usize) -> bool {
        !is_unique(&[x, y, z, x])
    }

    #[quickcheck]
    fn selftest_dir_rev_rev(d: Dir) -> bool {
        dir_rev(dir_rev(d)) == d
    }

    #[quickcheck]
    fn qc_mov_len1(d: Dir, l: Len) -> bool {
        (d.to_move() * l).len() == l
    }

    #[quickcheck]
    fn qc_mov_len2(d: Dir, l: Len, e: Dir, m: Len) -> bool {
        (d.to_move() * l + e.to_move() * m).len() == l + m || e == dir_rev(d)
    }

    #[quickcheck]
    fn qc_walk_len(p: Point, d: Dir, l: Len) -> bool {
        let obs_len = p.walk(d, l).count() as Len;
        obs_len == l
    }

    #[quickcheck]
    fn qc_walk_end(p: Point, d: Dir, l: Len) -> bool {
        if let Some(obs_end) = p.walk(d, l).last() {
            obs_end == endpt(p, d, l)
        } else {
            l == 0
        }
    }

    #[quickcheck]
    fn qc_walk_unique(p: Point, d: Dir, l: Len) -> bool {
        is_unique(p.walk(d, l))
    }

    #[quickcheck]
    fn qc_walk_rev(p: Point, d: Dir, l: Len) -> bool {
        let q = endpt(p, d, l);
        let e = dir_rev(d);
        let fwd: Vec<_> = iter::once(p).chain(p.walk(d, l)).collect();
        let rev: Vec<_> = iter::once(q).chain(q.walk(e, l)).collect();
        vec_rev(rev) == fwd
    }

    #[quickcheck]
    fn qc_walk_many_len(p: Point, dls: Vec<(Dir, Len)>) -> bool {
        if dls.len() == 0 {
            return true;
        }
        let obs_len = p.walk_many(dls.iter().cloned()).count() as Len;
        let exp_len = dls.iter().map(|&(_d, l)| l).sum();
        obs_len == exp_len
    }

    #[quickcheck]
    fn qc_walk2_unique(p: Point, d0: Dir, l0: Len, d1: Dir, l1: Len) -> bool {
        is_unique(p.walk_many([(d0, l0), (d1, l1)].iter().cloned())) || d1 == dir_rev(d0)
    }

    #[quickcheck]
    fn qc_walk_many_rev(p: Point, dls: Vec<(Dir, Len)>) -> bool {
        if dls.len() == 0 {
            return true;
        }
        let q = dls.iter().fold(p, |q, &(d, l)| endpt(q, d, l));
        let els = dls.clone().into_iter().rev().map(|(d, l)| (dir_rev(d), l));
        let fwd: Vec<_> = iter::once(p).chain(p.walk_many(dls)).collect();
        let rev: Vec<_> = iter::once(q).chain(q.walk_many(els)).collect();
        vec_rev(rev) == fwd
    }

    #[test]
    fn turnings() {
        assert_eq!(Dir::Up.to_move().turn_left(), Dir::Lf.to_move());
        assert_eq!(Dir::Lf.to_move().turn_left(), Dir::Dn.to_move());
        assert_eq!(Dir::Up.to_move().turn_right(), Dir::Rt.to_move());
        assert_eq!(Dir::Rt.to_move().turn_right(), Dir::Dn.to_move());
    }
}
