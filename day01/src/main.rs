fn fuel_req(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

fn main() {
    println!("Hello, world!");
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
