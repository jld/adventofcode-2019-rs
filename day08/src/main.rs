use std::io::{stdin, prelude::*};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const AREA: usize = WIDTH * HEIGHT;

fn layers<'d>(data: &'d [u8]) -> impl Iterator<Item = &'d [u8]> + 'd {
    assert!(data.len() % AREA == 0);
    let n = data.len() / AREA;
    (0..n).map(move |i| &data[(i * AREA)..((i + 1) * AREA)])
}

fn count_digit(layer: &[u8], digit: u8) -> usize {
    layer.iter().filter(|&&d| d == digit).count()
}

fn part1(data: &[u8]) -> usize {
    let thickest = layers(data).min_by_key(|layer| count_digit(layer, b'0')).unwrap();
    count_digit(&thickest, b'1') * count_digit(&thickest, b'2')
}

fn main() {
    let stdin = stdin();
    let data = stdin.lock().lines().next().expect("no input").expect("I/O error reading stdin");
    let data = data.into_bytes();
    println!("{}", part1(&data));
}
