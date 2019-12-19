use std::io::{stdin, BufRead};

fn fuel_req(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

fn main() {
    let stdin = stdin();
    
    let total: u32 =
        stdin.lock()
             .lines()
             .map(|rline| rline.expect("I/O error reading stdin"))
             .map(|line| u32::from_str_radix(&line, 10).unwrap())
             .map(fuel_req)
             .sum();

    println!("{}", total);
}

#[cfg(test)]
mod test {
    use super::fuel_req;

    #[test]
    fn spec() {
        assert_eq!(fuel_req(12), 2);
        assert_eq!(fuel_req(14), 2);
        assert_eq!(fuel_req(1969), 654);
        assert_eq!(fuel_req(100756), 33583);
    }
}
