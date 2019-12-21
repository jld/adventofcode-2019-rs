use std::io::{stdin, prelude::*};

use intcode::{Computer, Device, Word, IOError};

struct DiagDev;

impl Device for DiagDev {
    fn input(&mut self) -> Result<Word, IOError> {
        Ok(1)
    }
    fn output(&mut self, val: Word) -> Result<(), IOError> {
        Ok(println!("{}", val))
    }
}

fn main() {
    let stdin = stdin();
    let prog = stdin.lock().lines().next().expect("no input").expect("I/O error reading stdin");
    let mut cpu = Computer::from_str(&prog).expect("parse error");
    cpu.run(&mut DiagDev).expect("runtime error");
}
