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
    println!("Hello, world!");
}
