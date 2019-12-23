use std::io::{stdin, prelude::*};
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

fn fft1(src: &[Num], idx: usize) -> Num {
    src.iter()
       .zip(ThePattern::new(idx))
       .map(|(&a, b)| a * b)
       .sum::<Num>()
        .abs() % 10
}

fn fft(src: &[Num]) -> Vec<Num> {
    (0..src.len()).map(|i| fft1(src, i)).collect()
}

fn of_digit(ch: char) -> Num {
    let u = (ch as u32) - ('0' as u32);
    assert!(u < 10);
    u as Num
}

fn to_digit(i: Num) -> char {
    assert!(i >= 0 && i <= 9);
    (('0' as u8) + (i as u8)) as char
}
    
fn unpack(src: &str) -> Vec<Num> {
    src.chars().map(of_digit).collect()
}

fn repack(src: &[Num]) -> String {
    src.iter().cloned().map(to_digit).collect()
}

fn main() {
    let stdin = stdin();
    let line = stdin.lock().lines().next().expect("no input").expect("I/O error");
    let mut thing = unpack(&line);
    for _i in 0..100 {
        thing = fft(&thing);
    }
    println!("{}", repack(&thing[..8]));
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

    #[test]
    fn small_example() {
        let s0 = unpack("12345678");
        let s1 = fft(&s0);
        assert_eq!(s1, unpack("48226158"));
        let s2 = fft(&s1);
        assert_eq!(s2, unpack("34040438"));
        let s3 = fft(&s2);
        assert_eq!(s3, unpack("03415518"));
        let s4 = fft(&s3);
        assert_eq!(s4, unpack("01029498"));
    }

    #[test]
    fn test_unpack() {
        assert_eq!(unpack("0123456789"), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_repack() {
        assert_eq!(repack(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]), "0123456789".to_owned());
    }

    #[test]
    fn large_examples() {
        const CASES: &[(&str, &str)] = &[
            ("80871224585914546619083218645595", "24176176"),
            ("19617804207202209144916044189917", "73745418"),
            ("69317163492948606335995924319873", "52432133")];
        for &(src, dst) in CASES {
            let exp = unpack(dst);
            let mut got = unpack(src);
            for _i in 0..100 {
                got = fft(&got);
            }
            assert_eq!(&got[..8], &exp[..]);
        }
    }
}
