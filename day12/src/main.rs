use std::io::{stdin, prelude::*};
use std::iter::FromIterator;
use std::str::FromStr;
use std::ops::{Add, Sub, Deref};

type Num = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vec3(Num, Num, Num);

impl Vec3 {
    fn zero() -> Self {
        Vec3(0, 0, 0)
    }

    fn mag(self) -> Num {
        self.0.abs() + self.1.abs() + self.2.abs()
    }

    fn sgn(self) -> Self {
        Vec3(self.0.signum(), self.1.signum(), self.2.signum())
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Moon {
    pos: Vec3,
    vel: Vec3,
}

impl Moon {
    fn new(x: Num, y: Num, z: Num) -> Self {
        Self { pos: Vec3(x, y, z), vel: Vec3::zero() }
    }

    fn energy(&self) -> Num {
        self.pos.mag() * self.vel.mag()
    }

    fn gravity(&self, toward: &Moon) -> Vec3 {
        (toward.pos - self.pos).sgn()
    }

    fn accelerate(&mut self, dv: Vec3) {
        self.vel = self.vel + dv;
    }

    fn eulerize(&mut self) {
        self.pos = self.pos + self.vel
    }

    fn from_str(s: &str) -> Self {
        let mut nums = s.split_whitespace()
                        .map(|s| s.trim_matches(&[',', '<', '>', '=', 'x', 'y', 'z'][..]))
                        .filter(|s| !s.is_empty())
                        .map(|s| Num::from_str(s).expect("Moon::from_str: bad number"));
        let x = nums.next().expect("Moon::from_str: no x");
        let y = nums.next().expect("Moon::from_str: no y");
        let z = nums.next().expect("Moon::from_str: no z");
        assert!(nums.next().is_none(), "Moon::from_str: trailing garbage");
        Self::new(x, y, z)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Moons(Vec<Moon>);

impl Moons {
    fn step(&mut self) {
        let n = self.0.len();
        for i in 0..n {
            for j in 0..n {
                let dv = self.0[i].gravity(&self.0[j]);
                self.0[i].accelerate(dv);
            }
        }
        for i in 0..n {
            self.0[i].eulerize()
        }
    }

    fn energy(&self) -> Num {
        self.0.iter().map(|m| m.energy()).sum()
    }
}

impl Deref for Moons {
    type Target = [Moon];
    fn deref(&self) -> &[Moon] { &self.0 }
}

impl<'a> FromIterator<&'a str> for Moons {
    fn from_iter<I>(iter: I) -> Self
        where I: IntoIterator<Item = &'a str>
    {
        Moons(iter.into_iter().map(|s| Moon::from_str(s)).collect())
    }
}

impl FromIterator<String> for Moons {
    fn from_iter<I>(iter: I) -> Self
        where I: IntoIterator<Item = String>
    {
        Moons(iter.into_iter().map(|s| Moon::from_str(&s)).collect())
    }
}

fn main() {
    let stdin = stdin();
    let mut ms: Moons = stdin.lock().lines().map(|r| r.expect("I/O error reading stdin")).collect();
    for _ in 0..1000 {
        ms.step();
    }
    println!("{}", ms.energy());
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unit_gravity() {
        let gan = Moon::new(3, 0, 0);
        let cal = Moon::new(5, 0, 0);
        assert_eq!(gan.gravity(&cal), Vec3(1, 0, 0));
        assert_eq!(cal.gravity(&gan), Vec3(-1, 0, 0));
    }

    #[test]
    fn unit_euler() {
        let mut eur = Moon::new(1, 2, 3);
        eur.vel = Vec3(-2, 0, 3);
        eur.eulerize();
        assert_eq!(eur.pos, Vec3(-1, 2, 6));
    }

    fn example1() -> Moons {
        Moons::from_iter(["<x=-1, y=0, z=2>",
                          "<x=2, y=-10, z=-7>",
                          "<x=4, y=-8, z=8>",
                          "<x=3, y=5, z=-1>"]
                         .iter().cloned())
    }

    fn verify(ms: &Moons, stepno: usize, expected: &[(Vec3, Vec3)]) {
        assert_eq!(ms.len(), expected.len());
        for (i, &(p, v)) in expected.iter().enumerate() {
            assert_eq!(ms[i].pos, p, "bad position for moon {} at step {}", i, stepno);
            assert_eq!(ms[i].vel, v, "bad velocity for moon {} at step {}", i, stepno);
        }
    }

    #[test]
    fn ex1_first_steps() {
        let mut ms = example1();
        ms.step();
        verify(&ms, 1, &[(Vec3(2, -1, 1), Vec3(3, -1, -1)),
                         (Vec3(3, -7, -4), Vec3(1, 3, 3)),
                         (Vec3(1, -7, 5), Vec3(-3, 1, -3)),
                         (Vec3(2, 2, 0), Vec3(-1, -3, 1))]);
        ms.step();
        verify(&ms, 2, &[(Vec3(5, -3, -1), Vec3(3, -2, -2)),
                         (Vec3(1, -2, 2), Vec3(-2, 5, 6)),
                         (Vec3(1, -4, -1), Vec3(0, 3, -6)),
                         (Vec3(1, -4, 2), Vec3(-1, -6, 2))]);
        // Yes, I've been retyping these by hand.  If I cared I could
        // do regex things and copy/paste the whole text, but I don't.
    }

    #[test]
    fn ex1_final_energy() {
        let mut ms = example1();
        for _ in 0..10 {
            ms.step();
        }
        assert_eq!(ms.energy(), 179);
    }

    fn example2() -> Moons {
        Moons::from_iter(["<x=-8, y=-10, z=0>",
                          "<x=5, y=5, z=10>",
                          "<x=2, y=-7, z=3>",
                          "<x=9, y=-8, z=-3>"]
                         .iter().cloned())
    }

    #[test]
    fn ex2_final_energy() {
        let mut ms = example2();
        for _ in 0..100 {
            ms.step();
        }
        assert_eq!(ms.energy(), 1940);
    }
}
