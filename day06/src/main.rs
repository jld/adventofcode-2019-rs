use std::collections::{HashSet, HashMap};
use std::io::{stdin, prelude::*};

struct OrbDb(HashMap<String, String>);

impl OrbDb {
    fn new() -> Self { OrbDb(HashMap::new()) }

    fn add(&mut self, sup: &str, inf: &str) {
        let old_sup = self.0.insert(inf.to_owned(), sup.to_owned());
        assert!(old_sup.is_none());
    }

    fn add_line(&mut self, line: &str) {
        let mut things = line.splitn(2, ')');
        let sup = things.next().unwrap();
        let inf = things.next().unwrap();
        self.add(sup, inf);
    }

    fn sup(&self, whence: &str) -> Option<&str> {
        self.0.get(whence).map(|s| s as &str)
    }

    fn sups<'a>(&'a self, whence: &'a str) -> SupStream<'a> {
        SupStream { db: self, whence: Some(whence) }
    }

    fn num_sups(&self, whence: &str) -> usize {
        self.sups(whence).count()
    }

    fn count_stuff(&self) -> usize {
        self.0.keys().map(|k| self.num_sups(k)).sum()
    }

    fn xfers(&self, a: &str, b: &str) -> usize {
        let ah: HashSet<_> = self.sups(a).collect();
        let bh: HashSet<_> = self.sups(b).collect();
        ah.symmetric_difference(&bh).count()
    }
}

struct SupStream<'a> {
    db: &'a OrbDb,
    whence: Option<&'a str>,
}

impl<'a> Iterator for SupStream<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        self.whence = self.whence.and_then(|thence| self.db.sup(thence));
        self.whence
    }
}


fn main() {
    let mut db = OrbDb::new();
    let stdin = stdin();
    for line in stdin.lock().lines().map(|r| r.expect("I/O error reading stdin")) {
        db.add_line(&line);
    }
    println!("{}", db.count_stuff());
    println!("{}", db.xfers("YOU", "SAN"));
}

#[cfg(test)]
mod test {
    use super::OrbDb;

    fn example_orbs() -> OrbDb {
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
        return db;
    }

    #[test]
    fn example_count() {
        assert_eq!(example_orbs().count_stuff(), 42);
    }

    #[test]
    fn example_xfer() {
        let mut db = example_orbs();
        db.add_line("K)YOU");
        db.add_line("I)SAN");
        assert_eq!(db.xfers("YOU", "SAN"), 4);
    }

    #[test]
    fn example_xfer0() {
        let mut db = example_orbs();
        db.add_line("I)YOU");
        db.add_line("I)SAN");
        assert_eq!(db.xfers("YOU", "SAN"), 0);
    }
}
