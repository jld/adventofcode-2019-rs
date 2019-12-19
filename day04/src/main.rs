use std::env::args;
use std::str::FromStr;
use std::string::ToString;

fn has_adj(bs: &[u8]) -> bool {
    (0..(bs.len()-1)).any(|i| bs[i] == bs[i+1])
}

fn has_adjx(bs: &[u8]) -> bool {
    (0..(bs.len()-1)).any(|i| bs[i] == bs[i+1]
                          && bs.get(i+1) != bs.get(i+2)
                          && bs.get(i.wrapping_sub(1)) != bs.get(i))
}

fn mono_ndec(bs: &[u8]) -> bool {
    (0..(bs.len()-1)).all(|i| bs[i] <= bs[i+1])
}

fn pwd_check1(n: u32) -> bool {
    let bs = n.to_string().into_bytes();
    has_adj(&bs) && mono_ndec(&bs)
}

fn pwd_check2(n: u32) -> bool {
    let bs = n.to_string().into_bytes();
    has_adjx(&bs) && mono_ndec(&bs)
}

fn main() {
    let mut nums = args().skip(1).map(|s| u32::from_str(&s).unwrap());
    let lb = nums.next().expect("no lower bound");
    let ub = nums.next().expect("no upper bound");
    let n1: u32 = (lb..=ub).filter(|&n| pwd_check1(n)).map(|_| 1u32).sum();
    let n2: u32 = (lb..=ub).filter(|&n| pwd_check2(n)).map(|_| 1u32).sum();
    println!("{} {}", n1, n2);
}


#[cfg(test)]
mod test {
    use super::{has_adj, has_adjx, mono_ndec, pwd_check1, pwd_check2};

    #[test]
    fn test_adj() {
        assert!(has_adj(b"122345"));
        assert!(has_adj(b"111111"));
        assert!(!has_adj(b"123789"));
    }

    #[test]
    fn test_ndec() {
        assert!(mono_ndec(b"111123"));
        assert!(mono_ndec(b"135679"));
        assert!(mono_ndec(b"111111"));
        assert!(!mono_ndec(b"223450"));
    }

    #[test]
    fn test_pwd1() {
        assert!(pwd_check1(111111));
        assert!(!pwd_check1(223450));
        assert!(!pwd_check1(123789));
    }

    #[test]
    fn test_adjx() {
        assert!(has_adjx(b"112233"));
        assert!(!has_adjx(b"123444"));
        assert!(has_adjx(b"111122"));
    }

    #[test]
    fn test_pwd2() {
        assert!(pwd_check2(112233));
        assert!(!pwd_check2(123444));
        assert!(pwd_check2(111122));
    }
}
