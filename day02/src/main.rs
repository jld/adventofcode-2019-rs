use std::iter::IntoIterator;

type Num = usize;

struct Intcode {
    pc: usize,
    mem: Vec<Num>,
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

    pub fn get_mem(&self) -> &[Num] {
        &self.mem
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::{Num, Intcode};

    fn case1(before: &[Num], after: &[Num]) {
        let mut cpu = Intcode::new(before.iter().cloned());
        cpu.run();
        assert_eq!(cpu.get_mem(), after);
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
