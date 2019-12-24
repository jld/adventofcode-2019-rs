use std::io::{stdin, prelude::*};

use intcode::{Computer, Device, IOError, Word, ExecError, exec::ExecFault};
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
    backtrack: Option<Dir>,
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
            backtrack: None,
            oxygen: None,
            done: false
        }
    }

    fn print(&self) {
        let (low, high) = self.walls.bounding_box();
        for y in (low.y..=high.y).rev() {
            let mut line = String::new();
            for x in low.x..high.x {
                let xy = Point { x, y };
                let ch = if self.oxygen == Some(xy) { '$' }
                else if self.here == xy { '@' }
                else if self.walls.contains(xy) { '#' }
                else if self.open.contains(xy) { '.' }
                else { ' ' };
                line.push(ch);
            }
            println!("{}", line);
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
            let back = last.rev();
            assert!(self.backtrack.is_none());
            self.backtrack = Some(back);
            return Ok(cmd_of_dir(back));
        }

        // Must be done.
        assert_eq!(self.here, Point::origin());
        self.done = true;
        Err(IOError)
    }

    fn output(&mut self, val: Word) -> Result<(), IOError> {
        if val == 0 {
            assert!(self.backtrack.is_none());
            let attempt = self.trail.pop().unwrap().to_move();
            assert!(self.walls.insert(self.here + attempt));
        } else if val <= 2 {
            let dir = self.backtrack.take().unwrap_or_else(|| *self.trail.last().unwrap());
            self.here += dir.to_move();
            self.open.insert(self.here);
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
    let stdin = stdin();
    let prog = stdin.lock().lines().next().expect("no input").expect("I/O error reading stdin");
    let mut cpu = Computer::from_str(&prog).expect("parse error");

    let mut dev = SearchDev::new();
    match cpu.run(&mut dev) {
        Ok(()) => panic!("Unexpected CPU halt"),
        Err(ExecError { fault: ExecFault::IO(_), ..}) => assert!(dev.done),
        e @ Err(_) => e.expect("runtime error"),
    };
    dev.print();
    let ox = dev.oxygen.expect("gasp!");
    println!("Distance: {}", distance_to(&dev.open, Point::origin(), ox).expect("no path?"));
}
