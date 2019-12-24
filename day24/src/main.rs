mod part2;

use std::collections::HashSet;
use std::env::args;

type Grid = u32;
const ALL: Grid = 0b11111_11111_11111_11111_11111;
const SHRED: Grid = 0b01111_01111_01111_01111_01111;
const SHLED: Grid = 0b11110_11110_11110_11110_11110;

fn bugshift(prev: &[Grid], addend: Grid) -> Vec<Grid> {
    let n = prev.len();
    let mut next = vec![0; n + 1];
    for i in 0..n {
        next[i] |= prev[i] & !addend;
        next[i+1] |= prev[i] & addend;
    }
    next
}

fn bugadd(planes: &[Grid]) -> Vec<Grid> {
    planes.iter().fold(vec![ALL], |prev, addend| bugshift(&prev, *addend))
}

fn neighbors(grid: Grid) -> Vec<Grid> {
    bugadd(&[(grid >> 1) & SHRED,
             (grid << 1) & SHLED,
             grid >> 5,
             (grid << 5) & ALL])
}

fn bug_iter(grid: Grid) -> Grid {
    let counts = neighbors(grid);
    (grid & counts[1]) | (!grid & (counts[1] | counts[2]))
}


fn main() {
    let s = args().nth(1).expect("arg: in binary, backwards");
    let g0 = Grid::from_str_radix(&s, 2).expect("binary parse error");
    let mut g = g0;
    let mut seen = HashSet::new();
    loop {
        if !seen.insert(g) {
            println!("{}", g);
            break;
        }
        g = bug_iter(g);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        const TILES: &[Grid] =
            &[0b00001_00100_11001_01001_10000,
              0b00110_11011_10111_01111_01001,
              0b11101_01000_10000_10000_11111,
              0b10110_01101_11000_01111_00001,
              0b00011_00000_10011_10000_01111];
        for i in 1..TILES.len() {
            assert_eq!(bug_iter(TILES[i-1]), TILES[i]);
        }
    }
}
