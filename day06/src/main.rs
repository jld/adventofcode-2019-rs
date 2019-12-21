use std::collections::HashMap;

struct OrbDb(HashMap<String, Vec<String>>);

impl OrbDb {
    fn new() -> Self { OrbDb(HashMap::new()) }

    fn add(&mut self, sup: &str, inf: &str) {
        let ent = self.0.entry(sup.to_owned());
        ent.or_insert_with(|| vec![]).push(inf.to_owned());
    }

    fn add_line(&mut self, line: &str) {
        let mut things = line.splitn(2, ')');
        let sup = things.next().unwrap();
        let inf = things.next().unwrap();
        self.add(sup, inf);
    }

    fn xcount_infs(&self, depth: u64, whence: &str) -> u64 {
        let mut acc = depth;
        if let Some(infs) = self.0.get(whence) {
            for inf in infs {
                acc += self.xcount_infs(depth + 1, inf);
            }
        }
        return acc;
    }

    fn count_stuff(&self) -> u64 {
        self.xcount_infs(0, "COM")
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::OrbDb;

    #[test]
    fn example() {
        let mut db = OrbDb::new();
        for &line in &["COM)B",
                       "B)C",
                       "C)D",
                       "D)E",
                       "E)F",
                       "B)G",
                       "G)H",
                       "D)I",
                       "E)J",
                       "J)K",
                       "K)L"] {
            db.add_line(line);
        }
        assert_eq!(db.count_stuff(), 42);
    }
}
