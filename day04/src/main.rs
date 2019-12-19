use std::env::args;
use std::str::FromStr;
use std::string::ToString;

fn has_adj(bs: &[u8]) -> bool {
    (0..(bs.len()-1)).any(|i| bs[i] == bs[i+1])
}

fn mono_ndec(bs: &[u8]) -> bool {
    (0..(bs.len()-1)).all(|i| bs[i] <= bs[i+1])
}

fn pwd_check(n: u32) -> bool {
    let bs = n.to_string().into_bytes();
    has_adj(&bs) && mono_ndec(&bs)
}

fn main() {
    let mut nums = args().skip(1).map(|s| u32::from_str(&s).unwrap());
    let lb = nums.next().expect("no lower bound");
    let ub = nums.next().expect("no upper bound");
    let n: u32 = (lb..=ub).filter(|&n| pwd_check(n)).map(|_| 1u32).sum();
    println!("{}", n);
}


#[cfg(test)]
mod test {
    use super::{has_adj, mono_ndec, pwd_check};

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
    fn test_pwd() {
        assert!(pwd_check(111111));
        assert!(!pwd_check(223450));
        assert!(!pwd_check(123789));
    }
}
