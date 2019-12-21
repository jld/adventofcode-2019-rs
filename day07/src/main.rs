use intcode::{Computer, Device, IOError, Word};

fn permutations(n: usize) -> Vec<Vec<usize>> {
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

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::permutations;

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
}
