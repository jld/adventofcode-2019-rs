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
}
