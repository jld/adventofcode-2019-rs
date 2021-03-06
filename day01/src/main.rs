use itertools::iterate;
use std::env::args;
use std::io::{stdin, BufRead};
use std::process::exit;

fn fuel_req(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

fn fix_fuel(mass: u32) -> u32 {
    iterate(mass, |&m| fuel_req(m))
        .skip(1)
        .take_while(|&m| m > 0)
        .sum()
}

fn tmain<F>(fuel_fn: F)
    where F: Fn(u32) -> u32
{
    let stdin = stdin();
    
    let total: u32 =
        stdin.lock()
             .lines()
             .map(|rline| rline.expect("I/O error reading stdin"))
             .map(|line| u32::from_str_radix(&line, 10).unwrap())
             .map(fuel_fn)
             .sum();

    println!("{}", total);
}

fn main() {
    match args().nth(1).as_ref().map(|s| s as &str) { // sigh
        Some("1") => tmain(fuel_req),
        Some("2") => tmain(fix_fuel),
        _ => {
            eprintln!("Usage: {} <part #>", args().nth(0).unwrap());
            exit(1)
        }
    }
}

#[cfg(test)]
mod test {
    use super::{fuel_req, fix_fuel};

    #[test]
    fn spec1() {
        assert_eq!(fuel_req(12), 2);
        assert_eq!(fuel_req(14), 2);
        assert_eq!(fuel_req(1969), 654);
        assert_eq!(fuel_req(100756), 33583);
    }

    #[test]
    fn spec2() {
        assert_eq!(fix_fuel(14), 2);
        assert_eq!(fix_fuel(1969), 966);
        assert_eq!(fix_fuel(100756), 50346);
    }
}
