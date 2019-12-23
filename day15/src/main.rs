use intcode::{Computer, Device, IOError, Word};
use painting::{PointSet, Point, Dir};

const DIRS: &[Dir] = &[Dir::Up, Dir::Dn, Dir::Lf, Dir::Rt];

fn cmd_of_dir(d: Dir) -> Word {
    match d {
        Dir::Up => 1,
        Dir::Dn => 2,
        Dir::Lf => 3,
        Dir::Rt => 4,
    }
}

struct SearchDev {
    walls: PointSet,
    open: PointSet,
    here: Point,
    trail: Vec<Dir>,
    oxygen: Option<Point>,
    done: bool
}

impl SearchDev {
    fn new() -> Self {
        let here = Point::origin();
        let mut open = PointSet::new();
        open.insert(here);
        Self {
            walls: PointSet::new(),
            open,
            here,
            trail: vec![],
            oxygen: None,
            done: false
        }
    }
}

impl Device for SearchDev {
    fn input(&mut self) -> Result<Word, IOError> {
        // Explore?
        for &dir in DIRS {
            let unto = self.here + dir.to_move();
            if self.walls.contains(unto) || self.open.contains(unto) {
                continue;
            }
            self.trail.push(dir);
            return Ok(cmd_of_dir(dir));
        }

        // Backtrack?
        if let Some(last) = self.trail.pop() {
            return Ok(cmd_of_dir(last.rev()))
        }

        // Must be done.
        assert_eq!(self.here, Point::origin());
        self.done = true;
        Err(IOError)
    }

    fn output(&mut self, val: Word) -> Result<(), IOError> {
        if val == 0 {
            self.trail.pop().unwrap();
            assert!(self.walls.insert(self.here));
        } else if val <= 2 {
            self.here += self.trail.last().unwrap().to_move();
            if val == 2 {
                self.oxygen = Some(self.here);
            }
        } else {
            return Err(IOError);
        }
        Ok(())
    }
}

fn distance_to(open: &PointSet, start: Point, there: Point) -> Option<usize> {
    let mut open = open.clone();
    assert!(open.remove(start));
    let mut last = vec![start];
    let mut dist = 1;
    while !last.is_empty() {
        let mut next = vec![];
        for from in last {
            for &dir in DIRS {
                let unto = from + dir.to_move();
                if unto == there {
                    return Some(dist);
                }
                if !open.remove(unto) {
                    continue;
                }
                next.push(unto);
            }
        }
        last = next;
        dist += 1;
    }
    None
}

fn main() {
    println!("Hello, world!");
}
