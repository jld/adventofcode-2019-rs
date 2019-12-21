use std::collections::HashMap;
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
        self.sups(whence).map(|_| 1_usize).sum()
    }

    fn num_proper_sups(&self, whence: &str) -> usize {
        self.num_sups(whence) - 1
    } 

    fn count_stuff(&self) -> usize {
        self.0.keys().map(|k| self.num_proper_sups(k)).sum()
    }
}

struct SupStream<'a> {
    db: &'a OrbDb,
    whence: Option<&'a str>,
}

impl<'a> Iterator for SupStream<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        let rv = self.whence;
        if let Some(thence) = rv {
            self.whence = self.db.sup(thence);
        }
        rv
    }
}

fn main() {
    let mut db = OrbDb::new();
    let stdin = stdin();
    for line in stdin.lock().lines().map(|r| r.expect("I/O error reading stdin")) {
        db.add_line(&line);
    }
    println!("{}", db.count_stuff());
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
