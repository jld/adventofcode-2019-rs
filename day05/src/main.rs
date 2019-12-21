use std::env::args;
use std::io::{stdin, prelude::*};
use std::str::FromStr;

use intcode::{Computer, Device, Word, IOError};

struct DiagDev(Word);

impl Device for DiagDev {
    fn input(&mut self) -> Result<Word, IOError> {
        Ok(self.0)
    }
    fn output(&mut self, val: Word) -> Result<(), IOError> {
        Ok(println!("{}", val))
    }
}

fn main() {
    let sys_id = args().nth(1).map(|s| Word::from_str(&s).unwrap()).unwrap_or(1);
    let stdin = stdin();
    let prog = stdin.lock().lines().next().expect("no input").expect("I/O error reading stdin");
    let mut cpu = Computer::from_str(&prog).expect("parse error");
    cpu.run(&mut DiagDev(sys_id)).expect("runtime error");
}
