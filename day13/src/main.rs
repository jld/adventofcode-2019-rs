use std::collections::HashMap;
use std::io::{stdin, prelude::*};

use intcode::{Computer, Device, Word, IOError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Block,
    HPad,
    Ball,
}

impl Tile {
    fn from_word(w: Word) -> Option<Self> {
        Some (match w {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HPad,
            4 => Tile::Ball,
            _ => return None
        })
    }
}

struct ScreenDev {
    tiles: HashMap<(Word, Word), Tile>,
    cmd: Vec<Word>,
}

impl ScreenDev {
    fn new() -> Self {
        Self {
            tiles: HashMap::new(),
            cmd: vec![],
        }
    }
}

impl Device for ScreenDev {
    fn input(&mut self) -> Result<Word, IOError> { Err(IOError) }

    fn output(&mut self, val: Word) -> Result<(), IOError> {
        debug_assert!(self.cmd.len() < 3);
        self.cmd.push(val);
        Ok(if self.cmd.len() == 3 {
            let tile = Tile::from_word(self.cmd[2]).ok_or(IOError)?;
            self.tiles.insert((self.cmd[0], self.cmd[1]), tile);
            self.cmd.clear();
        })
    }
}

fn main() {
    // I really should clean up all this code duplication....
    let stdin = stdin();
    let prog = stdin.lock().lines().next().expect("no input").expect("I/O error reading stdin");
    let mut cpu = Computer::from_str(&prog).expect("parse error");

    let mut dev = ScreenDev::new();
    cpu.run(&mut dev).expect("runtime error");
    println!("{}", dev.tiles.iter().filter(|&(_xy, &t)| t == Tile::Block).count());
}
