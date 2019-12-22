use std::collections::HashMap;

use crate::geom::Point;

type BitSet = u64;

fn split(p: Point) -> (Point, BitSet) {
    let xh = p.x >> 3;
    let yh = p.y >> 3;
    let xl = p.x & 7;
    let yl = p.y & 7;
    let b = (yl << 3) + xl;
    (Point { x: xh, y: yh }, 1 << b)
}

#[derive(Clone, Debug)]
pub struct PointSet {
    inner: HashMap<Point, BitSet>
}

impl PointSet {
    pub fn new() -> Self {
        Self { inner: HashMap::new() }
    }

    pub fn contains(&self, p: Point) -> bool {
        let (key, mask) = split(p);
        self.inner.get(&key).cloned().unwrap_or(0) & mask != 0
    }

    pub fn insert(&mut self, p: Point) -> bool {
        let (key, mask) = split(p);
        let tile_ptr = self.inner.entry(key).or_insert(0);
        let old = *tile_ptr & mask != 0;
        *tile_ptr |= mask;
        return !old;
    }

    pub fn remove(&mut self, p: Point) -> bool {
        let (key, mask) = split(p);
        let tile_ptr = self.inner.entry(key).or_insert(0);
        let old = *tile_ptr & mask != 0;
        *tile_ptr &= !mask;
        return old;
    }

    pub fn len(&self) -> usize {
        self.inner.values().map(|&tile| tile.count_ones() as usize).sum()
    }
}

#[cfg(test)]
mod test {
    use super::PointSet;
    use crate::geom::Point;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn qc_empty_contains_nothing(p: Point) -> bool {
        !PointSet::new().contains(p)
    }

    #[quickcheck]
    fn qc_empty_insert_yes(p: Point) -> bool {
        PointSet::new().insert(p)
    }

    #[quickcheck]
    fn qc_empty_remove_no(p: Point) -> bool {
        !PointSet::new().remove(p)
    }

    #[quickcheck]
    fn qc_insert_contains_same(p: Point) -> bool {
        let mut ps = PointSet::new();
        ps.insert(p);
        ps.contains(p)
    }

    #[quickcheck]
    fn qc_insert_insert_same(p: Point) -> bool {
        let mut ps = PointSet::new();
        ps.insert(p);
        !ps.insert(p)
    }

    #[quickcheck]
    fn qc_insert_remove_same(p: Point) -> bool {
        let mut ps = PointSet::new();
        ps.insert(p);
        ps.remove(p)
    }

    #[quickcheck]
    fn qc_insert_contains_other(p: Point, q: Point) -> bool {
        let mut ps = PointSet::new();
        ps.insert(p);
        !ps.contains(q) || p == q
    }

    #[quickcheck]
    fn qc_insert_insert_other(p: Point, q: Point) -> bool {
        let mut ps = PointSet::new();
        ps.insert(p);
        ps.insert(q) || p == q
    }

    #[quickcheck]
    fn qc_insert_remove_other(p: Point, q: Point) -> bool {
        let mut ps = PointSet::new();
        ps.insert(p);
        !ps.remove(q) || p == q
    }

    #[quickcheck]
    fn qc_insert_remove_same_gone(p: Point) -> bool {
        let mut ps = PointSet::new();
        ps.insert(p);
        ps.remove(p);
        !ps.contains(p)
    }

    #[quickcheck]
    fn qc_insert_remove_other_still_there(p: Point, q: Point) -> bool {
        let mut ps = PointSet::new();
        ps.insert(p);
        ps.remove(q);
        ps.contains(p) || p == q
    }

    #[quickcheck]
    fn qc_insert2_contains_both(p: Point, q: Point) -> bool {
        let mut ps = PointSet::new();
        ps.insert(p);
        ps.insert(q);
        ps.contains(p) && ps.contains(q)
    }

    #[test]
    fn empty_len() {
        assert_eq!(PointSet::new().len(), 0);
    }

    #[quickcheck]
    fn qc_singleton_len(p: Point) -> bool {
        let mut ps = PointSet::new();
        ps.insert(p);
        ps.len() == 1
    }

    #[quickcheck]
    fn qc_double_insert_len(p: Point) -> bool {
        let mut ps = PointSet::new();
        ps.insert(p);
        ps.insert(p);
        ps.len() == 1
    }

    #[quickcheck]
    fn qc_singleton_emptied_len(p: Point) -> bool {
        let mut ps = PointSet::new();
        ps.insert(p);
        ps.remove(p);
        ps.len() == 0
    }

    #[quickcheck]
    fn qc_double_len(p: Point, q: Point) -> bool {
        let mut ps = PointSet::new();
        ps.insert(p);
        ps.insert(q);
        ps.len() == 2 || p == q
    }
}
