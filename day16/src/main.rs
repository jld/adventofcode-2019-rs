use std::iter::Iterator;

type Num = i32;

#[derive(Debug, Clone)]
struct ThePattern {
    scale: usize,
    low: usize,
    high: usize,
}

impl ThePattern {
    fn new(i: usize) -> Self {
        Self {
            scale: i + 1,
            low: 1,
            high: 0,
        }
    }
}

impl Iterator for ThePattern {
    type Item = Num;

    fn next(&mut self) -> Option<Num> {
        debug_assert!(self.low <= self.scale);
        if self.low == self.scale {
            self.low = 0;
            self.high += 1;
        }
        self.low += 1;
        self.high &= 3;
        Some(match self.high {
            0 => 0,
            1 => 1,
            2 => 0,
            3 => -1,
            _ => panic!("unreachable")
        })
    }
}
    

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn second_pattern() {
        let expected = vec![0, 1, 1, 0, 0, -1, -1, 0, 0, 1, 1, 0, 0, -1, -1];
        let actual: Vec<_> = ThePattern::new(1).take(expected.len()).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn other_patterns() {
        let p1: Vec<_> = ThePattern::new(0).take(8).collect();
        assert_eq!(p1, vec![1, 0, -1, 0, 1, 0, -1, 0]);

        let p3: Vec<_> = ThePattern::new(2).take(8).collect();
        assert_eq!(p3, vec![0, 0, 1, 1, 1, 0, 0, 0]);
    }

}
