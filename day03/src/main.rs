use std::collections::HashMap;
use std::io::{stdin, prelude::*};
use std::str::FromStr;

use painting::{Point,Dir,Len,PointSet};

fn parse_one(token: &str) -> (Dir, Len) {
    let c0 = token.chars().next().expect("empty move");
    let d = Dir::from_char(c0).expect("bad move char");
    // At this point the first char must be ASCII.
    (d, Len::from_str(&token[1..]).expect("bad number"))
}

fn parse(line: &str) -> Vec<(Dir, Len)> {
    line.split(',').map(parse_one).collect()
}

fn all_points(r: &[(Dir, Len)]) -> impl Iterator<Item = Point> + '_ {
    Point::origin().walk_many(r.iter().cloned())
}

fn all_crossings<'a, 'b>(r0: &'a[(Dir, Len)], r1: &'b[(Dir, Len)])
                         -> impl Iterator<Item = (usize, Point)> + 'b
{
    let mut ps = PointSet::new();
    for p in all_points(r0) {
        ps.insert(p);
    }
    all_points(r1)
        .enumerate()
        .map(|(i, p)| (i+1, p)) // Sigh
        .filter(move |&(_i, p)| ps.contains(p))
}

fn part1(s: &str, q: &str) -> Len {
    all_crossings(&parse(s), &parse(q))
        .map(|(_i, p)| (p - Point::origin()).len())
        .min()
        .unwrap()
}

fn part2(s: &str, q: &str) -> Len {
    let sp = parse(s);
    let qp = parse(q);
    let mut qls = HashMap::new();
    for (ql, pt) in all_crossings(&sp, &qp) {
        qls.entry(pt).or_insert(ql); // want the lowest if multiple
    }
    all_crossings(&qp, &sp)
        .map(|(sl, pt)| sl + *(qls.get(&pt).expect("asymmetrical crossings!?")))
        .min()
        .unwrap() as Len
}

fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines().map(|r| r.expect("I/O error reading stdin"));
    let l0 = lines.next().expect("no first input line");
    let l1 = lines.next().expect("no second input line");
    println!("{}", part1(&l0, &l1));
    println!("{}", part2(&l0, &l1));
}

#[cfg(test)]
mod test {
    use super::{part1, part2, parse, all_crossings};
    use painting::Point;

    #[test]
    fn spec1_diagrams() {
        assert_eq!(part1("R8,U5,L5,D3",
                         "U7,R6,D4,L4"),
                   6);
    }

    #[test]
    fn spec1_diag_whitebox() {
        let isects: Vec<_> =
            all_crossings(&parse("R8,U5,L5,D3"),
                          &parse("U7,R6,D4,L4"))
            .map(|(_i, pt)| pt)
            .collect();
        assert_eq!(isects, vec![Point { x: 6, y: 5 }, Point { x: 3, y: 3 }]);
    }

    #[test]
    fn spec1_line1() {
        assert_eq!(part1("R75,D30,R83,U83,L12,D49,R71,U7,L72",
                         "U62,R66,U55,R34,D71,R55,D58,R83"),
                   159);
    }

    #[test]
    fn spec1_line2() {
        assert_eq!(part1("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                         "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
                   135);
    }

    #[test]
    fn spec2_diagrams() {
        assert_eq!(part2("R8,U5,L5,D3",
                         "U7,R6,D4,L4"),
                   30);
    }

    #[test]
    fn spec2_line1() {
        assert_eq!(part2("R75,D30,R83,U83,L12,D49,R71,U7,L72",
                         "U62,R66,U55,R34,D71,R55,D58,R83"),
                   610);
    }

    #[test]
    fn spec2_line2() {
        assert_eq!(part2("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                         "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
                   410);
    }
}
