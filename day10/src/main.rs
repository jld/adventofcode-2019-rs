type Num = i32;

// FIXME may not work on negative numbers (but I probably don't need to care).
fn gcd(mut x: Num, mut y: Num) -> Num {
    while y != 0 {
        let z = x % y;
        x = y;
        y = z;
    }
    return x;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Angle {
    sx: Num,
    sy: Num,
}
impl Angle {
    fn new(x: Num, y: Num) -> Self {
        let scale = gcd(x.abs(), y.abs());
        assert_ne!(scale, 0);
        Self { sx: x/scale, sy: y/scale }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: Num,
    y: Num,
}
impl Point {
    fn new(x: Num, y: Num) -> Self { Self { x, y } }

    fn heading(self, other: Point) -> Option<Angle> {
        if self == other {
            None
        } else {
            Some(Angle::new(other.x - self.x, other.y - self.y))
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(3, 5), 1);
        assert_eq!(gcd(5, 3), 1);
        assert_eq!(gcd(5, 1), 1);
        assert_eq!(gcd(1, 5), 1);
        assert_eq!(gcd(6, 15), 3);
        assert_eq!(gcd(15, 6), 3);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(5, 0), 5);
        assert_eq!(gcd(0, 0), 0);
    }

    #[test]
    fn test_angle_eq() {
        assert_eq!(Angle::new(1, 2), Angle::new(2, 4));
        assert_eq!(Angle::new(-1, 2), Angle::new(-2, 4));
        assert_eq!(Angle::new(1, -2), Angle::new(2, -4));
        assert_eq!(Angle::new(-1, -2), Angle::new(-2, -4));
        assert_eq!(Angle::new(1, 0), Angle::new(100, 0));
    }

    #[test]
    fn test_angle_ne() {
        assert_ne!(Angle::new(1, 2), Angle::new(2, 1));
        assert_ne!(Angle::new(1, 2), Angle::new(-1, 2));
        assert_ne!(Angle::new(1, 2), Angle::new(-1, -2));
        assert_ne!(Angle::new(-1, 2), Angle::new(1, -2));
        assert_ne!(Angle::new(-1, 0), Angle::new(1, 0));
        assert_ne!(Angle::new(1, 0), Angle::new(0, 1));
    }

    #[test]
    fn test_heading() {
        let here = Point::new(3, 4);
        assert_eq!(here.heading(here), None);
        assert_eq!(here.heading(Point::new(1, 0)), here.heading(Point::new(2, 2)));
        assert_ne!(Point::new(2, 2).heading(here), here.heading(Point::new(2, 2)));
    }
}
