use std::convert::TryInto;
use std::io::{stdin, prelude::*};

use intcode::{Computer, Device, IOError, Word};

#[derive(Debug, Clone)]
struct CameraDev {
    chars: Vec<Vec<u8>>,
}

impl CameraDev {
    fn new() -> Self {
        Self {
            chars: vec![vec![]],
        }
    }

    fn trim(&mut self) {
        while self.chars.last().map_or(false, |l| l.is_empty()) {
            let _ = self.chars.pop();
        }
    }

    fn find_isects(&self) -> Vec<(usize, usize)> {
        assert!(!self.chars.is_empty());
        let height = self.chars.len();
        let width = self.chars[0].len();
        assert!(self.chars.iter().all(|l| l.len() == width));

        let mut acc = vec![];
        for y in 1..height-1 {
            for x in 1..width-1 {
                let is_cross = [(x-1, y), (x, y), (x+1, y), (x, y-1), (x, y+1)]
                    .iter()
                    .all(|&(xx, yy)| self.chars[yy][xx] == '#' as u8);
                if is_cross {
                    acc.push((x, y));
                }
            }
        }
        return acc;
    }
}

impl Device for CameraDev {
    fn input(&mut self) -> Result<Word, IOError> { Err(IOError) }

    fn output(&mut self, val: Word) -> Result<(), IOError> {
        Ok(if val == 10 {
            self.chars.push(vec![]);
        } else {
            self.chars.last_mut().unwrap().push(val.try_into().map_err(|_| IOError)?);
        })
    }
}

fn main() {
    let stdin = stdin();
    let prog = stdin.lock().lines().next().expect("no input").expect("I/O error reading stdin");
    let mut cpu = Computer::from_str(&prog).expect("parse error");

    let mut dev = CameraDev::new();
    cpu.run(&mut dev).expect("runtime error");
    for vline in &dev.chars {
        let line: String = vline.iter().map(|&c| c as char).collect();
        println!("{}", line);
    }
    dev.trim();
    let align = dev.find_isects();
    println!("{}", align.iter().map(|&(x, y)| x * y).sum::<usize>());
}
