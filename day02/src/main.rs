use std::env::args;
use std::io::{stdin, prelude::*};
use std::iter::IntoIterator;
use std::str::FromStr;
use std::process::exit;

type Num = usize;

struct Intcode {
    pub pc: usize,
    pub mem: Vec<Num>,
}

impl Intcode {
    pub fn new<I: IntoIterator<Item = Num>>(src: I) -> Self {
        Self {
            pc: 0,
            mem: src.into_iter().collect(),
        }
    }

    fn operate<Op>(&mut self, op: Op)
        where Op: Fn(Num, Num) -> Num {
        if let [a, b, c] = self.mem[(self.pc + 1)..=(self.pc + 3)] {
            self.mem[c] = op(self.mem[a], self.mem[b])
        } else {
            panic!("bus error reading insn positions")
        }
    }

    pub fn step(&mut self) -> bool {
        let stepped = match self.mem[self.pc] {
            1 => { self.operate(|a,b| a + b); true }
            2 => { self.operate(|a,b| a * b); true }
            99 => false,
            _ => panic!("bad opcode"),
        };
        if stepped {
            self.pc += 4
        }
        stepped
    }

    pub fn run(&mut self) {
        while self.step() { }
    }
}

fn compute(mem: impl IntoIterator<Item = Num>, noun: Num, verb: Num) -> Num {
    let mut cpu = Intcode::new(mem);
    cpu.mem[1] = noun;
    cpu.mem[2] = verb;
    cpu.run();
    return cpu.mem[0];
}

fn part1(mem: impl IntoIterator<Item = Num>) {
    println!("{}", compute(mem, 12, 2));
}

fn part2(mem: impl IntoIterator<Item = Num>) {
    const MOON: Num = 19690720;
    
    let mem: Vec<Num> = mem.into_iter().collect();
    for noun in 0..=99 {
        for verb in 0..=99 {
            if compute(mem.iter().cloned(), noun, verb) == MOON {
                println!("{}", 100 * noun + verb);
                return;
            }
        }
    }
    panic!("That's no moon!");
}

fn main() {
    let stdin = stdin();
    let mem =
        stdin.lock()
             .split(b',')
             .map(|r| r.expect("I/O error reading stdin"))
             .map(|b| String::from_utf8(b).expect("encoding error on stdin"))
             .map(|s| Num::from_str(s.trim())
                  .unwrap_or_else(|e| panic!("bad number {:?}: {}", s, e)));
    match args().nth(1).as_ref().map(|s| s as &str) { // sigh
        Some("1") => part1(mem),
        Some("2") => part2(mem),
        _ => {
            eprintln!("Usage: {} <part #>", args().nth(0).unwrap());
            exit(1)
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Num, Intcode};

    fn case1(before: &[Num], after: &[Num]) {
        let mut cpu = Intcode::new(before.iter().cloned());
        cpu.run();
        assert_eq!(&cpu.mem as &[Num], after);
    }

    #[test]
    fn spec1_line1() {
        case1(&[1,0,0,0,99], &[2,0,0,0,99]);
    }

    #[test]
    fn spec1_line2() {
        case1(&[2,3,0,3,99], &[2,3,0,6,99]);
    }

    #[test]
    fn spec1_line3() {
        case1(&[2,4,4,5,99,0], &[2,4,4,5,99,9801]);
    }

    #[test]
    fn spec1_line4() {
        case1(&[1,1,1,4,99,5,6,0,99], &[30,1,1,4,2,5,6,0,99]);
    }

}
