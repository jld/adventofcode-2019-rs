type Int = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Deal {
    Rev,
    Cut(Int),
    Inc(Int),
}

impl Deal {
    fn follow_card(self, size: Int, card: Int) -> Int {
        debug_assert!(card >= 0 && card < size);
        match self {
            Deal::Rev => size - 1 - card,
            Deal::Cut(off) => (card - off).rem_euclid(size),
            Deal::Inc(inc) => (card * inc) % size,
        }
    }
}

fn follow_card(size: Int, deals: &[Deal], card: Int) -> Int {
    deals.iter().fold(card, |card, deal| deal.follow_card(size, card))
}

fn main() {
    println!("Hello, world!");
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
