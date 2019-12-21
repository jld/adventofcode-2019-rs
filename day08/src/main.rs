use std::io::{stdin, prelude::*};
use std::str;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const AREA: usize = WIDTH * HEIGHT;

fn slices<'d>(data: &'d [u8], stride: usize) -> impl Iterator<Item = &'d [u8]> + 'd {
    assert!(data.len() % stride == 0);
    let n = data.len() / stride;
    (0..n).map(move |i| &data[(i * stride)..((i + 1) * stride)])
}

fn count_digit(layer: &[u8], digit: u8) -> usize {
    layer.iter().filter(|&&d| d == digit).count()
}

fn part1(data: &[u8]) -> usize {
    let thickest = slices(data, AREA).min_by_key(|layer| count_digit(layer, b'0')).unwrap();
    count_digit(&thickest, b'1') * count_digit(&thickest, b'2')
}

fn composite1(below: u8, above: u8) -> u8 {
    match above {
        b'0' | b'1' => above,
        b'2' => below,
        _ => panic!("bad pixel {:x}", above)
    }
}

fn undercomp(below: &[u8], above: &mut[u8]) {
    for (i, app) in above.iter_mut().enumerate() {
        *app = composite1(below[i], *app)
    }
}

fn display(slice: &[u8]) {
    for row in slices(slice, WIDTH) {
        println!("{}", str::from_utf8(row).unwrap());
    }
}

fn part2(data: &[u8]) -> Vec<u8> {
    let mut image = vec![b'2'; AREA];
    for layer in slices(data, AREA) {
        undercomp(layer, &mut image);
    }
    image
}

fn main() {
    let stdin = stdin();
    let data = stdin.lock().lines().next().expect("no input").expect("I/O error reading stdin");
    let data = data.into_bytes();
    println!("{}", part1(&data));
    display(&part2(&data));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_comp() {
        let mut data = vec![];
        for plane in vec![b"0222", b"1122", b"2212", b"0000"] {
            data.extend_from_slice(plane);
            data.append(&mut vec![b'2'; AREA - 4]);
        }
        let img = part2(&data);
        assert_eq!(&img[0..4], b"0110");
    }
}
