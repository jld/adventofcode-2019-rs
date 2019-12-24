use std::env::args;
use std::io::{stdin, stdout, Stdout, prelude::*};

use intcode::{Computer, Device, IOError, Word};

struct SpringProgDev {
    in_buf: Vec<u8>,
    stdout: Stdout,
}

impl SpringProgDev {
    pub fn new(lines: &[&str]) -> Self {
        let mut buf = vec![];
        for line in lines {
            buf.extend_from_slice(line.as_bytes());
            buf.push('\n' as u8);
        }
        buf.reverse();

        Self {
            in_buf: buf,
            stdout: stdout(),
        }
    }
}

impl Device for SpringProgDev {
    fn input(&mut self) -> Result<Word, IOError> {
        self.in_buf.pop().map(|b| b as Word).ok_or(IOError)
    }

    fn output(&mut self, val: Word) -> Result<(), IOError> {
        if val >= 0 && val < 128 {
            let buf = [val as u8];
            assert_eq!(self.stdout.write(&buf).expect("error writing stdout"), 1);
        } else {
            write!(&mut self.stdout, "Number: {}\n", val).expect("error writing stdout");
        }
        Ok(())
    }
}

fn main() {
    let stdin = stdin();
    let prog = stdin.lock().lines().next().expect("no input").expect("I/O error reading stdin");
    let mut cpu = Computer::from_str(&prog).expect("parse error");

    let walkp = args().nth(1).map_or(true, |s| s != "run");

    let spring = if walkp {
        &["NOT A J",
          "NOT B T",
          "OR T J",
          "NOT C T",
          "OR T J",
          "AND D J",
          "WALK"][..]
    } else {
        &["NOT A J",
          "NOT B T",
          "OR T J",
          "NOT C T",
          "OR T J",
          "AND D J",
          "NOT E T",
          "NOT T T",
          "OR H T",
          "AND T J",
          "RUN"][..]
    };

    let mut dev = SpringProgDev::new(spring);
    cpu.run(&mut dev).expect("runtime error");
}
