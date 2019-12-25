use std::io::{stdin, prelude::*};
use std::str::FromStr;

type Int = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Deal {
    Rev,
    Cut(Int),
    Inc(Int),
}

impl Deal {
    fn follow_card(self, size: Int, card: Int) -> Int {
        self.compile(size).apply(size, card)
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

    fn compile(self, size: Int) -> LPerm {
        match self {
            Deal::Rev =>
                LPerm { m: -1, b: size - 1 },
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
    fn apply(self, size: Int, card: Int) -> Int {
        (card * self.m + self.b).rem_euclid(size)
    }
}

fn follow_card(size: Int, deals: &[Deal], card: Int) -> Int {
    deals.iter().fold(card, |card, deal| deal.follow_card(size, card))
}

fn main() {
    let stdin = stdin();
    let deals: Vec<_> = stdin.lock()
                             .lines()
                             .map(|r| Deal::from_str(&r.expect("I/O error reading stdin")))
                             .collect();

    println!("{}", follow_card(10007, &deals, 2019));
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
