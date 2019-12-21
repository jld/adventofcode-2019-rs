use std::env::args;
use std::io::{stdin, prelude::*};
use std::iter::IntoIterator;
use std::str::FromStr;
use std::process::exit;

use intcode::{Computer, Word};

fn compute(mem: Vec<Word>, noun: Word, verb: Word) -> Word {
    let mut cpu = Computer::new(mem.into_iter().collect());
    cpu.write(1, noun).unwrap();
    cpu.write(2, verb).unwrap();
    cpu.run(&mut ()).unwrap();
    return cpu.read(0).unwrap();
}

fn part1(mem: Vec<Word>) {
    println!("{}", compute(mem, 12, 2));
}

fn part2(mem: Vec<Word>) {
    const MOON: Word = 19690720;
    
    for noun in 0..=99 {
        for verb in 0..=99 {
            if compute(mem.clone(), noun, verb) == MOON {
                println!("{}", 100 * noun + verb);
                return;
            }
        }
    }
    panic!("That's no moon!");
}

fn main() {
    let stdin = stdin();
    let mem: Vec<Word> =
        stdin.lock()
             .split(b',')
             .map(|r| r.expect("I/O error reading stdin"))
             .map(|b| String::from_utf8(b).expect("encoding error on stdin"))
             .map(|s| Word::from_str(s.trim())
                  .unwrap_or_else(|e| panic!("bad number {:?}: {}", s, e)))
             .collect();
    match args().nth(1).as_ref().map(|s| s as &str) { // sigh
        Some("1") => part1(mem),
        Some("2") => part2(mem),
        _ => {
            eprintln!("Usage: {} <part #>", args().nth(0).unwrap());
            exit(1)
        }
    }
}
