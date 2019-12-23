use std::collections::HashMap;

type Amount = u64;

#[derive(Debug, Clone)]
struct Thing {
    quantity: Amount,
    substance: String,
}

#[derive(Debug, Clone)]
struct Reaction {
    output: Thing,
    inputs: Vec<Thing>,
}

#[derive(Debug, Clone)]
struct Book {
    recipes: HashMap<String, Reaction>,
}

#[derive(Debug, Clone)]
struct Bench {
    ore_used: Amount,
    leftovers: HashMap<String, Amount>,
}

fn div_round_up(n: Amount, d: Amount) -> Amount {
    (n + d - 1) / d
}

impl Book {
    fn new() -> Self { Self { recipes: HashMap::new() } }

    fn add(&mut self, recipe: Reaction) {
        let old = self.recipes.insert(recipe.output.substance.clone(), recipe);
        if let Some(wtf) = old {
            panic!("duplicate recipe for {}", &wtf.output.substance);
        }
    }
}

impl Bench {
    fn new() -> Self {
        Self {
            ore_used: 0,
            leftovers: HashMap::new(),
        }
    }

    fn obtain(&mut self, book: &Book, mut product: Thing) {
        if product.substance == "ORE" {
            self.ore_used += product.quantity;
            return;
        }
        if let Some(already_have) = self.leftovers.get_mut(&product.substance) {
            if *already_have >= product.quantity {
                *already_have -= product.quantity;
                return;
            }
            product.quantity -= std::mem::replace(already_have, 0);
        }
        let recipe = book.recipes.get(&product.substance)
                                 .unwrap_or_else(|| panic!("no recipe for {}", product.substance));
        let scale = div_round_up(product.quantity, recipe.output.quantity);
        let leftover = scale * recipe.output.quantity - product.quantity;
        if leftover > 0 {
            *(self.leftovers.entry(product.substance.clone()).or_insert(0)) += leftover;
        }
        for input in &recipe.inputs {
            let mut scaled_input = input.clone();
            scaled_input.quantity *= scale;
            self.obtain(book, scaled_input);
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    type RefThing<'r> = (Amount, &'r str);
    type HandBook<'r> = &'r[(&'r[RefThing<'r>], RefThing<'r>)];

    fn comp_thing(refthing: RefThing<'_>) -> Thing {
        Thing { quantity: refthing.0, substance: refthing.1.to_owned() }
    }

    fn compile(handbook: HandBook<'_>) -> Book {
        let mut book = Book::new();
        for hand_rec in handbook {
            book.add(Reaction {
                output: comp_thing(hand_rec.1),
                inputs: hand_rec.0.iter().map(|&th| comp_thing(th)).collect(),
            })
        }
        book
    }

    #[test]
    fn example1() {
        let book = compile(
            &[(&[(10, "ORE")], (10, "A")),
              (&[(1, "ORE")], (1, "B")),
              (&[(7, "A"), (1, "B")], (1, "C")),
              (&[(7, "A"), (1, "C")], (1, "D")),
              (&[(7, "A"), (1, "D")], (1, "E")),
              (&[(7, "A"), (1, "E")], (1, "FUEL"))]);
        let mut bench = Bench::new();
        bench.obtain(&book, Thing { quantity: 1, substance: "FUEL".to_owned() });
        assert_eq!(bench.ore_used, 31);
    }
}
