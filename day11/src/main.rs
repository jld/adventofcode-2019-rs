use std::io::{stdin, prelude::*};

use intcode::{Computer, Device, IOError, Word};
use painting::{PointSet, Point, Move, Dir};

#[derive(Debug, Clone)]
struct PaintDev {
    canvas: PointSet,
    mask: PointSet,
    turtle: Point,
    heading: Move,
    ldisc: LDisc
}

#[derive(Debug, Clone)]
enum LDisc {
    Paint,
    Turn,
}

impl PaintDev {
    fn new() -> Self {
        Self {
            canvas: PointSet::new(),
            mask: PointSet::new(),
            turtle: Point::origin(),
            heading: Dir::Up.to_move(),
            ldisc: LDisc::Paint,
        }
    }

    fn mask_size(&self) -> usize {
        self.mask.len()
    }

    fn blanch(&mut self) {
        self.canvas.insert(Point::origin());
    }

    fn print(&self) {
        let (low, high) = self.canvas.bounding_box();
        for y in (low.y..=high.y).rev() {
            let mut line = String::new();
            for x in low.x..high.x {
                line.push(if self.canvas.contains(Point { x, y }) { '#' } else { '.' })
            }
            println!("{}", line);
        }
    }
}

impl Device for PaintDev {
    fn input(&mut self) -> Result<Word, IOError> {
        Ok(if self.canvas.contains(self.turtle) { 1 } else { 0 })
    }

    fn output(&mut self, val: Word) -> Result<(), IOError> {
        Ok(self.ldisc = match self.ldisc {
            LDisc::Paint => {
                match val {
                    0 => self.canvas.remove(self.turtle),
                    1 => self.canvas.insert(self.turtle),
                    _ => return Err(IOError)
                };
                self.mask.insert(self.turtle);
                LDisc::Turn
            }
            LDisc::Turn => {
                self.heading = match val {
                    0 => self.heading.turn_left(),
                    1 => self.heading.turn_right(),
                    _ => return Err(IOError)
                };
                self.turtle += self.heading;
                LDisc::Paint
            }
        })
    }
}

fn main() {
    let stdin = stdin();
    let prog = stdin.lock().lines().next().expect("no input").expect("I/O error reading stdin");
    let mut cpu0 = Computer::from_str(&prog).expect("parse error");
    let mut cpu1 = cpu0.clone();

    let mut dev = PaintDev::new();
    cpu0.run(&mut dev).expect("runtime error");
    println!("{}", dev.mask_size());

    dev = PaintDev::new();
    dev.blanch();
    cpu1.run(&mut dev).expect("runtime error");
    dev.print();
}

#[cfg(test)]
mod test {
    use super::*;
    use intcode::{Device, Word};
    use painting::{Point, Dir};

    fn supply(dev: &mut impl Device, outs:&[Word]) {
        for &val in outs {
            dev.output(val).unwrap()
        }
    }

    #[test]
    fn the_example() {
        let mut dev = PaintDev::new();
        assert_eq!(dev.input(), Ok(0));

        supply(&mut dev, &[1,0]);
        assert_eq!(dev.input(), Ok(0));
        assert_eq!(dev.canvas.len(), 1);
        assert!(dev.canvas.contains(Point::origin()));
        assert_eq!(dev.mask.len(), 1);
        assert!(dev.mask.contains(Point::origin()));
        assert_eq!(dev.turtle, Point{ x: -1, y: 0 });
        assert_eq!(dev.heading, Dir::Lf.to_move());

        supply(&mut dev, &[0,0]);
        assert_eq!(dev.canvas.len(), 1);
        assert_eq!(dev.mask.len(), 2);
        assert_eq!(dev.turtle, Point{ x: -1, y: -1 });

        supply(&mut dev, &[1,0, 1,0]);
        assert_eq!(dev.input(), Ok(1));
        assert_eq!(dev.canvas.len(), 3);
        assert_eq!(dev.mask.len(), 4);
        assert_eq!(dev.turtle, Point::origin());
        assert_eq!(dev.heading, Dir::Up.to_move());

        supply(&mut dev, &[0,1, 1,0, 1,0]);
        assert_eq!(dev.canvas.len(), 4);
        assert_eq!(dev.mask.len(), 6);
    }
}
