use intcode::{Computer, Device, IOError, Word};

fn permutations(n: Word) -> Vec<Vec<Word>> {
    if n == 0 {
        return vec![vec![]];
    }
    let ps = permutations(n - 1);
    let mut qs = vec![];
    for i in 0..n {
        for p in &ps {
            let mut q = vec![i];
            for &j in p {
                q.push(if j >= i { j + 1 } else { j });
            }
            qs.push(q);
        }
    }
    return qs;
}

struct AmpDev {
    inputs: Vec<Word>,
    output: Option<Word>,
}

impl AmpDev {
    fn new(phase: Word, last_out: Word) -> Self {
        Self {
            inputs: vec![last_out, phase],
            output: None,
        }
    }
}

impl Device for AmpDev {
    fn input(&mut self) -> Result<Word, IOError> {
        self.inputs.pop().ok_or(IOError)
    }
    fn output(&mut self, val: Word) -> Result<(), IOError> {
        if self.output.is_some() {
            Err(IOError)
        } else {
            Ok(self.output = Some(val))
        }
    }
}

fn amplify_one(cpu: &Computer, phase: Word, last_out: Word) -> Word {
    let mut cpu = cpu.clone();
    let mut dev = AmpDev::new(phase, last_out);
    cpu.run(&mut dev).expect("magic smoke escaped");
    dev.output.expect("no output?")
}

fn amplify(cpu: &Computer, phases: &[Word]) -> Word {
    phases.iter().fold(0, |last_out, &phase| amplify_one(cpu, phase, last_out))
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::{permutations, amplify};
    use intcode::{Computer, Word};

    #[test]
    fn small_perms() {
        assert_eq!(permutations(0), vec![vec![]]);
        assert_eq!(permutations(1), vec![vec![0]]);
        assert_eq!(permutations(2), vec![vec![0, 1], vec![1, 0]]);
        assert_eq!(permutations(3), vec![vec![0, 1, 2], vec![0, 2, 1],
                                         vec![1, 0, 2], vec![1, 2, 0],
                                         vec![2, 0, 1], vec![2, 1, 0]]);
    }

    // Could also verify larger n abstractly: check length is n! and
    // each length is n and unique and each element is unique and all
    // ints are in range.  But this is probably good enough.

    fn thrustify(prog: Vec<Word>, best: Word, when: &[Word]) {
        let cpu = Computer::new(prog);
        assert_eq!(amplify(&cpu, when), best);
        for other in permutations(when.len() as Word) {
            assert!(amplify(&cpu, &other) <= best, "input {:?} exceeded {:?}", other, best)
        }
    }

    #[test]
    fn example1() {
        thrustify(vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0],
                  43210, &[4,3,2,1,0])
    }

    #[test]
    fn example2() {
        thrustify(vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,
                       101,5,23,23,1,24,23,23,4,23,99,0,0],
                  54321, &[0,1,2,3,4]);
    }

    #[test]
    fn example3() {
        thrustify(vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
                       1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0],
                  65210, &[1,0,4,3,2]);
    }
}
