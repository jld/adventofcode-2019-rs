use std::io::{stdin, prelude::*};
use std::iter::Iterator;

type Digit = u8;
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

fn last_digit(n: Num) -> Digit {
    (n.abs() % 10) as Digit
}

fn fft1(src: &[Digit], idx: usize) -> Digit {
    last_digit(src.iter()
               .zip(ThePattern::new(idx))
               .map(|(&d, c)| (d as Num) * c)
               .sum())
}

fn fft(src: &[Digit]) -> Vec<Digit> {
    (0..src.len()).map(|i| fft1(src, i)).collect()
}

// Notice anything about the bottom half of the example matrices?
fn tri_sum(src: &[Digit]) -> Vec<Digit> {
    let mut last = 0;
    let mut acc = vec![];
    for &d in src.iter() {
        last += d;
        if last > 9 {
            last -= 10;
        }
        acc.push(last);
    }
    return acc;
}

fn part2(src: &[Digit]) -> Option<Vec<Digit>> {
    let whence = to_actual_num(&src[..7]);
    let n = src.len();

    // Expect a Christmas miracle:
    if whence < 5000 * n {
        return None;
    }
    assert!(whence < 10000 * n);
    let how_many = 10000 - (whence / n);
    let size = 10000 * n - whence;

    let mut buf = vec![];
    let mut one = src.to_owned();
    one.reverse();
    for _i in 0..how_many {
        buf.extend_from_slice(&one);
    }
    std::mem::drop(one);
    buf.truncate(size);
    assert_eq!(buf.len(), size);

    // This could be optimized further: when tri_sum gets to the
    // second repetition of the sequence, it's just doing the same
    // thing it did the first time, but with an offset (the sum of the
    // repeating unit) added.  This generalizes to arbitrarily long
    // repetition and multiple layers of tri_sum.
    //
    // But that's not needed here; this already runs in seconds
    // without rustc optimization and milliseconds with it, nor is
    // memory usage a problem.
    for _i in 0..100 {
        buf = tri_sum(&buf);
    }
    let mut result = buf[buf.len()-8..].to_owned();
    result.reverse();
    Some(result)
}

fn of_digit(ch: char) -> Digit {
    let u = (ch as u32) - ('0' as u32);
    assert!(u < 10);
    u as Digit
}

fn to_digit(i: Digit) -> char {
    assert!(i <= 9);
    (('0' as u8) + i) as char
}

fn to_actual_num(ds: &[Digit]) -> usize {
    let mut acc = 0;
    for &d in ds.iter() {
        acc *= 10;
        acc += d as usize;
    }
    return acc;
}
    
fn unpack(src: &str) -> Vec<Digit> {
    src.chars().map(of_digit).collect()
}

fn repack(src: &[Digit]) -> String {
    src.iter().cloned().map(to_digit).collect()
}

fn main() {
    let stdin = stdin();
    let line = stdin.lock().lines().next().expect("no input").expect("I/O error");
    let message = unpack(&line);
    let mut p1buf = fft(&message);
    for _i in 1..100 {
        p1buf = fft(&p1buf);
    }
    println!("{}", repack(&p1buf[..8]));
    if let Some(result) = part2(&message) {
        println!("{}", repack(&result));
    } else {
        println!("Xmas miracle failed. )-:");
    }
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

    #[test]
    fn test_tri_sum() {
        assert_eq!(tri_sum(&[8, 7, 6, 5]), vec![8, 5, 1, 6]);
        assert_eq!(tri_sum(&[8, 5, 1, 6]), vec![8, 3, 4, 0]);
        assert_eq!(tri_sum(&[8, 3, 4, 0]), vec![8, 1, 5, 5]);
        assert_eq!(tri_sum(&[8, 1, 5, 5]), vec![8, 9, 4, 9]);
    }

    #[test]
    fn part2_examples() {
        const CASES: &[(&str, &str)] = &[
            ("03036732577212944063491565474664", "84462026"),
            ("02935109699940807407585447034323", "78725270"),
            ("03081770884921959731165446850517", "53553731")];
        for &(src, dst) in CASES {
            let src = unpack(src);
            let exp = unpack(dst);
            assert_eq!(&part2(&src).unwrap()[..], &exp[..]);
        }
    }
}
