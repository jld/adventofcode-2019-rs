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
