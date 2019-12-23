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

#[derive(Debug, Clone)]
struct CmdBuf {
    buf: Vec<Word>,
}

impl CmdBuf {
    fn new() -> Self { Self { buf: vec![] } }

    fn handle(&mut self, val: Word) -> Result<Option<(Word, Word, Tile)>, IOError> {
        debug_assert!(self.buf.len() < 3);
        self.buf.push(val);
        Ok(if self.buf.len() == 3 {
            let x = self.buf[0];
            let y = self.buf[1];
            let t = Tile::from_word(self.buf[2]).ok_or(IOError)?;
            self.buf.clear();
            Some((x, y, t))
        } else {
            None
        })
    }
}

#[derive(Debug, Clone)]
struct ScreenDev {
    tiles: HashMap<(Word, Word), Tile>,
    cmd: CmdBuf
}

impl ScreenDev {
    fn new() -> Self {
        Self {
            tiles: HashMap::new(),
            cmd: CmdBuf::new(),
        }
    }
}

impl Device for ScreenDev {
    fn input(&mut self) -> Result<Word, IOError> { Err(IOError) }

    fn output(&mut self, val: Word) -> Result<(), IOError> {
        Ok(if let Some((x, y, tile)) = self.cmd.handle(val)? {
            self.tiles.insert((x, y), tile);
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
