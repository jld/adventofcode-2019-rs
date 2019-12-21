use std::cmp::{PartialOrd, Ord, Ordering};
use std::collections::{HashSet, BTreeMap};
use std::io::{stdin, prelude::*};
use std::iter::FromIterator;
use std::mem;

type Num = i32;

// FIXME may not work on negative numbers (but I probably don't need to care).
fn gcd(mut x: Num, mut y: Num) -> Num {
    while y != 0 {
        let z = x % y;
        x = y;
        y = z;
    }
    return x;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Angle {
    sx: Num,
    sy: Num,
}
impl Angle {
    #[allow(dead_code)]
    fn new(x: Num, y: Num) -> Self {
        Self::new_plus(x, y).0
    }

    fn new_plus(x: Num, y: Num) -> (Self, Num) {
        let scale = gcd(x.abs(), y.abs());
        assert_ne!(scale, 0);
        (Self { sx: x/scale, sy: y/scale }, scale)
    }

    fn is_leftern(self) -> bool {
        self.sx < 0 || self.sx == 0 && self.sy > 0
    }
}

impl Ord for Angle {
    // The basic idea here is to exactly compare two ratios (y/x) by
    // cross-multiplying.  But that's not quite it, because that's
    // comparing slopes of lines and would equate opposite headings,
    // so it's necessary to split the space in half.  Conveniently,
    // the slope ordering puts up/down before up-right/down-left.
    fn cmp(&self, other: &Angle) -> Ordering {
        let skey = (self.is_leftern(), self.sy * other.sx);
        let okey = (other.is_leftern(), other.sy * self.sx);
        skey.cmp(&okey)
    }
}

impl PartialOrd for Angle {
    fn partial_cmp(&self, other: &Angle) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: Num,
    y: Num,
}
impl Point {
    fn new(x: Num, y: Num) -> Self { Self { x, y } }

    fn heading(self, other: Point) -> Option<Angle> {
        self.heading_plus(other).map(|(a,_s)| a)
    }

    fn heading_plus(self, other: Point) -> Option<(Angle, Num)> {
        if self == other {
            None
        } else {
            Some(Angle::new_plus(other.x - self.x, other.y - self.y))
        }
    }
}

#[derive(Debug, Clone)]
struct Map(HashSet<Point>);

impl Map {
    fn detectable(&self, whence: Point) -> usize {
        let hs: HashSet<Angle> = self.0.iter().filter_map(|&ast| whence.heading(ast)).collect();
        hs.len()
    }

    fn best_spot(&self) -> Point {
        self.0.iter()
              .cloned()
              .max_by_key(|&ast| self.detectable(ast))
              .unwrap()
    }

    #[cfg(test)]
    fn from_strs(strs: &[&str]) -> Self {
        strs.iter().cloned().map(|s| s.to_owned()).collect()
    }
}

#[derive(Debug, Clone)]
struct LaserMap(BTreeMap<Angle, BTreeMap<Num, Point>>);

impl LaserMap {
    fn new(map: &Map, whence: Point) -> Self {
        // Yes, the point could be rematerialized from the angle and scale and origin.
        // But I don't feel like it.
        let mut radiance: BTreeMap<Angle, BTreeMap<Num, Point>> = BTreeMap::new();

        for &there in &map.0 {
            if let Some((angle, scale)) = whence.heading_plus(there) {
                radiance.entry(angle).or_insert_with(BTreeMap::new).insert(scale, there);
            }
        }

        LaserMap(radiance)
    }

    fn firing_order(&self) -> Vec<Point> {
        let mut targets = vec![];
        let mut rays: Vec<_> = self.0.values().map(|ray| ray.values()).collect();
        while !rays.is_empty() {
            let old_rays = mem::replace(&mut rays, vec![]);
            for mut ray in old_rays {
                if let Some(&zap) = ray.next() {
                    targets.push(zap);
                    rays.push(ray);
                }
            }
        }
        targets
    }
}

impl FromIterator<String> for Map {
    fn from_iter<I>(iter: I) -> Self
        where I: IntoIterator<Item = String>
    {
        let mut this = Map(HashSet::new());
        for (y, line) in iter.into_iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                match ch {
                    '#' => assert!(this.0.insert(Point::new(x as Num, y as Num))),
                    '.' => (),
                    _ => panic!("bad map char {:?}", ch)
                }
            }
        }
        this
    }
}

fn main() {
    let stdin = stdin();
    let map: Map = stdin.lock().lines().map(|r| r.expect("I/O error reading stdin")).collect();
    let best = map.best_spot();
    println!("{}", map.detectable(best));
    let zaps = LaserMap::new(&map, best).firing_order();
    if let Some(bet) = zaps.get(199) {
        println!("{}", bet.x * 100 + bet.y);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(3, 5), 1);
        assert_eq!(gcd(5, 3), 1);
        assert_eq!(gcd(5, 1), 1);
        assert_eq!(gcd(1, 5), 1);
        assert_eq!(gcd(6, 15), 3);
        assert_eq!(gcd(15, 6), 3);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(5, 0), 5);
        assert_eq!(gcd(0, 0), 0);
    }

    #[test]
    fn test_angle_eq() {
        assert_eq!(Angle::new(1, 2), Angle::new(2, 4));
        assert_eq!(Angle::new(-1, 2), Angle::new(-2, 4));
        assert_eq!(Angle::new(1, -2), Angle::new(2, -4));
        assert_eq!(Angle::new(-1, -2), Angle::new(-2, -4));
        assert_eq!(Angle::new(1, 0), Angle::new(100, 0));
    }

    #[test]
    fn test_angle_ne() {
        assert_ne!(Angle::new(1, 2), Angle::new(2, 1));
        assert_ne!(Angle::new(1, 2), Angle::new(-1, 2));
        assert_ne!(Angle::new(1, 2), Angle::new(-1, -2));
        assert_ne!(Angle::new(-1, 2), Angle::new(1, -2));
        assert_ne!(Angle::new(-1, 0), Angle::new(1, 0));
        assert_ne!(Angle::new(1, 0), Angle::new(0, 1));
    }

    #[test]
    fn test_heading() {
        let here = Point::new(3, 4);
        assert_eq!(here.heading(here), None);
        assert_eq!(here.heading(Point::new(1, 0)), here.heading(Point::new(2, 2)));
        assert_ne!(Point::new(2, 2).heading(here), here.heading(Point::new(2, 2)));
    }

    #[test]
    fn test_rotation() {
        const POINTS: [(Num, Num); 16] =
            [(0, -2), (1, -2), (2, -2), (2, -1),
             (2, 0), (2, 1), (2, 2), (1, 2),
             (0, 2), (-1, 2), (-2, 2), (-2, 1),
             (-2, 0), (-2, -1), (-2, -2), (-1, -2)];
        for i in 0..16 {
            let l = Angle::new(POINTS[i].0, POINTS[i].1);
            assert!(!(l < l), "{:?} < {:?}", POINTS[i], POINTS[i]);
            assert!(!(l > l), "{:?} > {:?}", POINTS[i], POINTS[i]);
            for j in (i+1)..16 {
                let r = Angle::new(POINTS[j].0, POINTS[j].1);
                assert!(l < r, "{:?} !< {:?}", POINTS[i], POINTS[j]);
                assert!(r > l, "{:?} !> {:?}", POINTS[j], POINTS[i]);
            }
        }
    }

    #[test]
    fn example1() {
        let map = Map::from_strs(
            &[".#..#",
              ".....",
              "#####",
              "....#",
              "...##"]);

        assert_eq!(map.detectable(Point::new(3, 4)), 8);
        assert_eq!(map.best_spot(), Point::new(3, 4));
        assert_eq!(map.detectable(Point::new(0, 2)), 6);
        assert_eq!(map.detectable(Point::new(4, 2)), 5);
        assert_eq!(map.detectable(Point::new(4, 4)), 7);
    }

    #[test]
    fn example2() {
        let map = Map::from_strs(
            &["......#.#.",
              "#..#.#....",
              "..#######.",
              ".#.#.###..",
              ".#..#.....",
              "..#....#.#",
              "#..#....#.",
              ".##.#..###",
              "##...#..#.",
              ".#....####"]);
        assert_eq!(map.detectable(Point::new(5, 8)), 33);
        assert_eq!(map.best_spot(), Point::new(5, 8));
    }

    #[test]
    fn example3() {
        let map = Map::from_strs(
            &["#.#...#.#.",
              ".###....#.",
              ".#....#...",
              "##.#.#.#.#",
              "....#.#.#.",
              ".##..###.#",
              "..#...##..",
              "..##....##",
              "......#...",
              ".####.###."]);
        assert_eq!(map.detectable(Point::new(1, 2)), 35);
        assert_eq!(map.best_spot(), Point::new(1, 2));
    }

    #[test]
    fn example4() {
        let map = Map::from_strs(
            &[".#..#..###",
              "####.###.#",
              "....###.#.",
              "..###.##.#",
              "##.##.#.#.",
              "....###..#",
              "..#.#..#.#",
              "#..#.#.###",
              ".##...##.#",
              ".....#.#.."]);
        assert_eq!(map.detectable(Point::new(6, 3)), 41);
        assert_eq!(map.best_spot(), Point::new(6, 3));
    }

    #[test]
    fn example5() {
        let map = Map::from_strs(
            &[".#..##.###...#######",
              "##.############..##.",
              ".#.######.########.#",
              ".###.#######.####.#.",
              "#####.##.#.##.###.##",
              "..#####..#.#########",
              "####################",
              "#.####....###.#.#.##",
              "##.#################",
              "#####.##.###..####..",
              "..######..##.#######",
              "####.##.####...##..#",
              ".#####..#.######.###",
              "##...#.##########...",
              "#.##########.#######",
              ".####.#.###.###.#.##",
              "....##.##.###..#####",
              ".#.#.###########.###",
              "#.#.#.#####.####.###",
              "###.##.####.##.#..##"]);
        assert_eq!(map.detectable(Point::new(11, 13)), 210);
        assert_eq!(map.best_spot(), Point::new(11, 13));
    }

    #[test]
    fn big_firing_order() {
        let map = Map::from_strs(
            &[".#..##.###...#######",
              "##.############..##.",
              ".#.######.########.#",
              ".###.#######.####.#.",
              "#####.##.#.##.###.##",
              "..#####..#.#########",
              "####################",
              "#.####....###.#.#.##",
              "##.#################",
              "#####.##.###..####..",
              "..######..##.#######",
              "####.##.####...##..#",
              ".#####..#.######.###",
              "##...#.##########...",
              "#.##########.#######",
              ".####.#.###.###.#.##",
              "....##.##.###..#####",
              ".#.#.###########.###",
              "#.#.#.#####.####.###",
              "###.##.####.##.#..##"]);
        let zaps = LaserMap::new(&map, Point::new(11, 13)).firing_order();
        const EXPECTED: &[(usize, Num, Num)] =
            &[(1, 11, 12),
              (2, 12, 1),
              (3, 12, 2),
              (10, 12, 8),
              (20, 16, 0),
              (50, 16, 9),
              (100, 10, 16),
              (199, 9, 6),
              (200, 8, 2),
              (201, 10, 9),
              (299, 11, 1)];
        assert_eq!(zaps.len(), 299);
        for &(nth, x, y) in EXPECTED {
            assert_eq!(zaps[nth - 1], Point::new(x, y),
                       "bad value for zap #{}", nth);
        }
    }
}
