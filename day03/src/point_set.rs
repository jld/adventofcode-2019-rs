use std::collections::HashMap;

use crate::geom::{Point, Coord};

type BitSet = u64;

fn split(p: Point) -> (Point, BitSet) {
    let xh = p.x >> 3;
    let yh = p.y >> 3;
    let xl = p.x & 7;
    let yl = p.y & 7;
    let b = (yl << 3) + xl;
    (Point { x: xh, y: yh }, 1 << b)
}

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
        let grid_ptr = self.inner.entry(key).or_insert(0);
        let old = *grid_ptr & mask != 0;
        *grid_ptr |= mask;
        return old;
    }
    
    pub fn remove(&mut self, p: Point) -> bool {
        let (key, mask) = split(p);
        let grid_ptr = self.inner.entry(key).or_insert(0);
        let old = *grid_ptr & mask != 0;
        *grid_ptr &= !mask;
        return old;
    }
}
