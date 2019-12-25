use std::io::{stdin, prelude::*};
use std::str::FromStr;

type Int = i128; // Need at least 96 bits, or else multiplication tricks.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Deal {
    Rev,
    Cut(Int),
    Inc(Int),
}

impl Deal {
    #[allow(dead_code)]
    fn follow_card(self, size: Int, card: Int) -> Int {
        self.compile().apply(size, card)
    }

    fn from_str(s: &str) -> Self {
        if s == "deal into new stack" {
            return Deal::Rev;
        }
        let mut tokens = s.rsplitn(2, ' ');
        let num = Int::from_str(tokens.next().expect("empty line?")).expect("int parse error");
        let cmd = tokens.next().expect("no space?");
        if cmd == "cut" {
            Deal::Cut(num)
        } else if cmd == "deal with increment" {
            Deal::Inc(num)
        } else {
            panic!("unknown card command {:?}", cmd)
        }
    }

    fn compile(self) -> LPerm {
        match self {
            Deal::Rev =>
                LPerm { m: -1, b: -1 },
            Deal::Cut(off) =>
                LPerm { m: 1, b: -off },
            Deal::Inc(inc) =>
                LPerm { m: inc, b: 0 },
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct LPerm {
    m: Int,
    b: Int,
}

impl LPerm {
    fn id() -> LPerm { LPerm { m: 1, b: 0 } }

    fn apply(self, size: Int, card: Int) -> Int {
        (card * self.m + self.b).rem_euclid(size)
    }

    fn add(size: Int, p0: LPerm, p1: LPerm) -> LPerm {
        // (c * m0 + b0) * m1 + b1 = c * m0 * m1 + b0 * m1 + b1
        LPerm {
            m: (p0.m * p1.m) % size,
            b: (p0.b * p1.m + p1.b) % size,
        }
    }

    fn mul(size: Int, p: LPerm, n: u64) -> LPerm {
        if n == 0 {
            LPerm::id()
        } else if n == 1 {
            p
        } else {
            let almost = Self::mul(size, Self::add(size, p, p), n/2);
            if n % 2 == 0 {
                almost
            } else {
                Self::add(size, almost, p)
            }
        }
    }

    fn neg(size: Int, p: LPerm) -> LPerm {
        let nm = mmi(size, p.m);
        LPerm {
            m: nm,
            b: (-p.b * nm) % size
        }
    }
}

fn mmi(size: Int, thing: Int) -> Int {
    let (mut r0, mut r1, mut t0, mut t1) = (size, thing, 0, 1);
    while r1 != 0 {
        let r2 = r0.rem_euclid(r1);
        let t2 = t0 - t1 * r0.div_euclid(r1);
        r0 = r1;
        r1 = r2;
        t0 = t1;
        t1 = t2;
    }
    assert_eq!(r0, 1, "The shell of the world is cracked.  The bird flies to Abraxas.");
    return t0;
}

fn compile_many(size: Int, deals: &[Deal]) -> LPerm {
    deals.iter().fold(LPerm::id(), |lp, deal| LPerm::add(size, lp, deal.compile()))
}

fn follow_card(size: Int, deals: &[Deal], card: Int) -> Int {
    compile_many(size, deals).apply(size, card)
}

fn main() {
    let stdin = stdin();
    let deals: Vec<_> = stdin.lock()
                             .lines()
                             .map(|r| Deal::from_str(&r.expect("I/O error reading stdin")))
                             .collect();
    println!("{}", follow_card(10007, &deals, 2019));

    const BIG_SIZE: Int = 119315717514047;
    const BIG_REPS: u64 = 101741582076661;

    let lp = compile_many(BIG_SIZE, &deals);
    let lpx = LPerm::mul(BIG_SIZE, lp, BIG_REPS);
    let card = LPerm::neg(BIG_SIZE, lpx).apply(BIG_SIZE, 2020);
    assert_eq!(lpx.apply(BIG_SIZE, card), 2020);
    println!("{}", card);
}

#[cfg(test)]
mod test {
    use super::*;

    fn testcase(deals: &[Deal], result: &[Int]) {
        let size = result.len() as Int;
        for (i, &card) in result.iter().enumerate() {
            assert_eq!(follow_card(size, deals, card), i as Int, "misplaced card {}", card);
        }
    }

    #[test]
    fn example1() {
        testcase(&[Deal::Inc(7), Deal::Rev, Deal::Rev], &[0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }

    #[test]
    fn example2() {
        testcase(&[Deal::Cut(6), Deal::Inc(7), Deal::Rev], &[3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    }

    #[test]
    fn example3() {
        testcase(&[Deal::Inc(7), Deal::Inc(9), Deal::Cut(-2)], &[6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
    }

    #[test]
    fn example4() {
        testcase(&[Deal::Rev,
                   Deal::Cut(-2),
                   Deal::Inc(7),
                   Deal::Cut(8),
                   Deal::Cut(-4),
                   Deal::Inc(7),
                   Deal::Cut(3),
                   Deal::Inc(9),
                   Deal::Inc(3),
                   Deal::Cut(-1)],
                 &[9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }
}
