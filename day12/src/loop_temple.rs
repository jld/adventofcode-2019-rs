use crate::{Num, Vec3, Moons};

use std::cmp::max;
use std::ops::Mul;

pub(crate) const MOONS: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Facet {
    ps: [Num; MOONS],
    vs: [Num; MOONS],
}

impl Facet {
    pub(crate) fn extract<MF>(ms: &Moons, mf: MF) -> Self
        where MF: Fn(Vec3) -> Num
    {
        assert_eq!(ms.len(), MOONS);
        let mut this = Self { ps: [0; MOONS], vs: [0; MOONS] };
        for i in 0..MOONS {
            this.ps[i] = mf(ms[i].pos);
            this.vs[i] = mf(ms[i].vel);
        }
        this
    }

    pub(crate) fn step(&mut self) {
        for i in 0..MOONS {
            for j in 0..MOONS {
                if i != j {
                    self.vs[i] += (self.ps[j] - self.ps[i]).signum();
                }
            }
        }
        for i in 0..MOONS {
            self.ps[i] += self.vs[i];
        }
    }

    pub(crate) fn steps(&mut self, n: u64) {
        for _ in 0..n {
            self.step();
        }
    }

    pub(crate) fn rho_probe(&self) -> u64 {
        let mut fast = self.clone();
        let mut slow = self.clone();
        for i in 1u64.. {
            slow.step();
            fast.steps(2);
            if slow == fast {
                return i;
            }
        }
        panic!("u64 overflow");
    }

    pub(crate) fn find_tail(&self, period: u64) -> u64 {
        let mut leader = self.clone();
        let mut follower = self.clone();
        leader.steps(period);
        for i in 0u64.. {
            if leader == follower {
                return i;
            }
            leader.step();
            follower.step();
        }
        panic!("u64 overflow");
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Rho {
    pub(crate) tail: u64,
    pub(crate) period: u64,
}

impl Rho {
    pub(crate) fn compute(s: &Facet) -> Self {
        let r0 = s.rho_probe();
        let mut sr0 = s.clone();
        sr0.steps(r0);
        let period = sr0.rho_probe();
        Rho { period, tail: s.find_tail(period) }
    }

    pub(crate) fn total(self) -> u64 {
        self.tail + self.period
    }
}

fn gcd(mut x: u64, mut y: u64) -> u64 {
    while y != 0 {
        let z = x % y;
        x = y;
        y = z;
    }
    return x;
}

fn lcm(x: u64, y: u64) -> u64 {
    x * (y / gcd(x, y))
}

impl Mul for Rho {
    type Output = Rho;
    fn mul(self, other: Rho) -> Rho {
        Rho {
            tail: max(self.tail, other.tail),
            period: lcm(self.period, other.period),
        }
    }
}
