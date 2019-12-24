use std::borrow::Borrow;
use std::collections::{HashSet, HashMap};
use std::iter::IntoIterator;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile {
    x: u8, // 0..5
    y: u8, // 0..5
    z: i8, // -100..=100 probably
}

impl Tile {
    pub fn new(x: u8, y: u8, z: i8) -> Option<Self> {
        assert!(z != -128);
        if x < 5 && y < 5 && !(x == 2 && y == 2) {
            Some(Self { x, y, z })
        } else {
            None
        }
    }

    pub fn neighbors(self, out: &mut Vec<Tile>) {
        out.clear();

        // Intra
        out.extend(Tile::new(self.x.wrapping_sub(1), self.y, self.z));
        out.extend(Tile::new(self.x.wrapping_add(1), self.y, self.z));
        out.extend(Tile::new(self.x, self.y.wrapping_sub(1), self.z));
        out.extend(Tile::new(self.x, self.y.wrapping_add(1), self.z));

        // Outer
        if self.x == 0 { out.push(Tile::new(1, 2, self.z.wrapping_sub(1)).unwrap()); }
        if self.x == 4 { out.push(Tile::new(3, 2, self.z.wrapping_sub(1)).unwrap()); }
        if self.y == 0 { out.push(Tile::new(2, 1, self.z.wrapping_sub(1)).unwrap()); }
        if self.y == 4 { out.push(Tile::new(2, 3, self.z.wrapping_sub(1)).unwrap()); }
         
        // Inner
        if self.x == 1 && self.y == 2 {
            out.extend((0..5).map(|oy| Tile::new(0, oy, self.z.wrapping_add(1)).unwrap()));
        }
        if self.x == 3 && self.y == 2 {
            out.extend((0..5).map(|oy| Tile::new(4, oy, self.z.wrapping_add(1)).unwrap()));
        }
        if self.x == 2 && self.y == 1 {
            out.extend((0..5).map(|ox| Tile::new(ox, 0, self.z.wrapping_add(1)).unwrap()));
        }
        if self.x == 2 && self.y == 3 {
            out.extend((0..5).map(|ox| Tile::new(ox, 4, self.z.wrapping_add(1)).unwrap()));
        }
    }

    pub fn xy_bit(self) -> u32 {
        1u32 << (self.y * 5 + self.x)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid(HashSet<Tile>);

impl Deref for Grid {
    type Target = HashSet<Tile>;
    fn deref(&self) -> &HashSet<Tile> { &self.0 }
}

impl Grid {
    pub fn new() -> Self { Grid(HashSet::new()) }

    pub fn add(&mut self, x: u8, y: u8, z: i8) {
        self.0.insert(Tile::new(x, y, z).expect("bad coordinates"));
    }

    pub fn neighbors(&self) -> HashMap<Tile, u8> {
        let mut map = HashMap::new();
        let mut nbuf = vec![];
        for &tile in &self.0 {
            let _ = map.entry(tile).or_insert(0u8);
            tile.neighbors(&mut nbuf);
            for &neigh in &nbuf {
                *(map.entry(neigh).or_insert(0u8)) += 1;
            }
        }
        map
    }

    pub fn next(&mut self) {
        for (tile, nn) in self.neighbors() {
            if self.0.contains(&tile) {
                if nn != 1 {
                    self.0.remove(&tile);
                }
            } else {
                if nn == 1 || nn == 2 {
                    self.0.insert(tile);
                }
            }
        }
    }

    pub fn add_line(&mut self, y: u8, z: i8, line: &str) {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                self.add(x as u8, y, z)
            }
        }
    }

    pub fn add_plane<Line, Plane>(&mut self, z: i8, plane: Plane)
        where Line: Borrow<str>,
              Plane: IntoIterator<Item = Line>
    {
        for (y, line) in plane.into_iter().enumerate() {
            self.add_line(y as u8, z, line.borrow())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn tile0(n: u8) -> Tile {
        let u = n - 1;
        Tile::new(u % 5, u / 5, 0).unwrap()
    }

    fn tile1(c: char) -> Tile {
        let u = c as u8 - 'A' as u8;
        Tile::new(u % 5, u / 5, 1).unwrap()
    }

    fn tile_bag(near: Tile) -> Vec<Tile> {
        let mut buf = vec![];
        near.neighbors(&mut buf);
        buf
    }

    fn neigh(near: Tile, exp: &[Tile]) {
        let bag = tile_bag(near);
        assert_eq!(bag.len(), exp.len());
        for tile in exp {
            assert!(bag.contains(tile), "missing {:?} near {:?}", tile, near);
        }
    }

    #[test]
    fn neigh19() {
        neigh(tile0(19), &[tile0(14), tile0(18), tile0(20), tile0(24)]);
    }

    #[test]
    fn neigh_g() {
        neigh(tile1('G'), &[tile1('B'), tile1('F'), tile1('H'), tile1('L')]);
    }

    #[test]
    fn neigh_d() {
        neigh(tile1('D'), &[tile0(8), tile1('C'), tile1('E'), tile1('I')]);
    }

    #[test]
    fn neigh_e() {
        neigh(tile1('E'), &[tile0(8), tile1('D'), tile0(14), tile1('J')]);
    }

    #[test]
    fn neigh14() {
        neigh(tile0(14), &[tile0(9), tile1('E'), tile1('J'), tile1('O'),
                           tile1('T'), tile1('Y'), tile0(15), tile0(19)]);
    }

    const INIT: &[&[&str]] = 
        &[&["....#",
            "#..#.",
            "#.?##",
            "..#..",
            "#...."]];

    const AFTER1: &[&[&str]] =
        &[&[".....",
            "..#..",
            "..?#.",
            "..#..",
            "....."],

          &["#..#.",
            "####.",
            "##?.#",
            "##.##",
            ".##.."],

          &["....#",
            "....#",
            "....#",
            "....#",
            "#####"]];

    const AFTER10: &[&[&str]] =
        &[&["..#..",
            ".#.#.",
            "..?.#",
            ".#.#.",
            "..#.."],

          &["...#.",
            "...##",
            "..?..",
            "...##",
            "...#."],

          &["#.#..",
            ".#...",
            "..?..",
            ".#...",
            "#.#.."],

          &[".#.##",
            "....#",
            "..?.#",
            "...##",
            ".###."],

          &["#..##",
            "...##",
            "..?..",
            "...#.",
            ".####"],

          &[".#...",
            ".#.##",
            ".#?..",
            ".....",
            "....."],

          &[".##..",
            "#..##",
            "..?.#",
            "##.##",
            "#####"],

          &["###..",
            "##.#.",
            "#.?..",
            ".#.##",
            "#.#.."],

          &["..###",
            ".....",
            "#.?..",
            "#....",
            "#...#"],

          &[".###.",
            "#..#.",
            "#.?..",
            "##.#.",
            "....."],

          &["####.",
            "#..#.",
            "#.?#.",
            "####.",
            "....."]];

    fn check_grid(blame: &str, got: &Grid, exp: &Grid) {
        if got != exp {
            for &pt in exp.iter().filter(|ppp| !got.contains(*ppp)) {
                eprintln!("{}: expected {:?} but didn't get it", blame, pt);
            }
            for &pt in got.iter().filter(|ppp| !exp.contains(*ppp)) {
                eprintln!("{}: got {:?} but didn't expect it", blame, pt);
            }
            panic!("{}: discrepancies; see above", blame);
        }
        
    }

    fn parse_grid(offset: i8, stuff: &[&[&str]]) -> Grid {
        let mut grid = Grid::new();
        for (i, plane) in stuff.iter().enumerate() {
            grid.add_plane(i as i8 - offset, plane.iter().cloned());
        }
        grid
    }

    #[test]
    fn part2_evolution() {
        let mut grid = parse_grid(0, INIT);
        let after1 = parse_grid(1, AFTER1);
        let after10 = parse_grid(5, AFTER10);

        grid.next();
        check_grid("After 1", &grid, &after1);
        for _i in 1..10 {
            grid.next();
        }
        check_grid("After 10", &grid, &after10);
        assert_eq!(grid.len(), 99);
    }

    use ::quickcheck::*;
    use quickcheck_macros::quickcheck;

    impl Arbitrary for Tile {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            loop {
                const Z_BOUND: i8 = 6;
                let (x, y, z): (u8, u8, i8) = Arbitrary::arbitrary(g);
                if let Some(tile) = Tile::new(x % 5, y % 5, z % Z_BOUND) {
                    return tile;
                }
            }
        }
    }

    #[quickcheck]
    fn qc_neighbor_symmetric(t: Tile) -> bool {
        for neigh in tile_bag(t) {
            if !tile_bag(neigh).contains(&t) {
                eprintln!("{:?} -> {:?} but not reverse", t, neigh);
                return false;
            }
        }
        return true;
    }

    #[quickcheck]
    fn qc_neighbor_count(t: Tile) -> bool {
        let ns = tile_bag(t);
        ns.len() == 4 || ns.len() == 8
    }

    fn octonary(t: Tile) -> bool {
        tile_bag(t).len() == 8
    }

    #[quickcheck]
    fn qc_topology_2nd(t: Tile) -> bool {
        if octonary(t) {
            tile_bag(t).iter().all(|&u| !octonary(u))
        } else {
            let os = tile_bag(t).iter().filter(|&&u| octonary(u)).count();
            os == 1 || os == 2
        }
    }

    #[quickcheck]
    fn qc_buf_reuse(t: Tile, u: Tile) -> bool {
        let mut b0 = vec![];
        t.neighbors(&mut b0);
        u.neighbors(&mut b0);

        let mut b1 = vec![];
        u.neighbors(&mut b1);
        
        b0 == b1
    }
}
