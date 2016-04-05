extern crate rand;
use rand::{weak_rng, Rng, XorShiftRng, SeedableRng};

use std::env;
use std::io::{self, Read};
use std::collections::HashMap;

struct MarkovTable<'a> {
    table: HashMap<(&'a str, &'a str), Vec<&'a str>>,
    seed: Option<[u32; 4]>,
}

impl<'a> MarkovTable<'a> {
   fn new(string: &str) -> MarkovTable {
        MarkovTable {
            table: MarkovTable::parse(string),
            seed: None,
        }
    }

    fn tokenize(string: &str) -> Vec<&str> {
        let tokens: Vec<&str> = string.split_whitespace().collect();
        tokens
    }

    fn parse(string: &str) -> HashMap<(&str, &str), Vec<&str>> {
        let mut table: HashMap<(&str, &str), Vec<&str>> = HashMap::new();

        let words = MarkovTable::tokenize(string);
        let grouped_words = words.windows(3);

        for word_group in grouped_words {
            let prefix = (word_group[0], word_group[1]);
            let suffix = word_group[2];

            let new_suffix = match table.get(&prefix) {
                Some(existing_words) => {
                    let mut new_words = existing_words.clone();
                    new_words.push(suffix);
                    new_words
                },
                None => vec![suffix],
            };

            table.insert(prefix, new_suffix);
        }

        table
    }

    fn seed(&'a mut self, seed: [u32; 4]) -> &'a mut MarkovTable {
        self.seed = Some(seed);
        self
    }

    fn rng(&self) -> XorShiftRng {
        match &self.seed {
            &Some(seed) => SeedableRng::from_seed(seed),
            &None => weak_rng(),
        }
    }

    fn generate(&mut self, max_words: u32) -> String {
        let table = &self.table;
        let mut rng = self.rng();

        let mut possible_prefixes: Vec<&(&str, &str)> = table.keys().collect();
        possible_prefixes.sort();
        let prefix: &(&str, &str) = rng.choose(&possible_prefixes)
            .expect("couldn't choose initial prefix");
        let &(mut word1, mut word2) = prefix;

        let mut result = word1.to_string() + " " + word2;

        for _ in 1..(max_words - 1) {
            match table.get(&(word1, word2)) {
                Some(suffixes) => {
                    word1 = word2;
                    word2 = rng.choose(&suffixes).expect("couldn't choose suffix");
                    result = result + " " + word2;
                },
                None => {
                    break;
                },
            }
        }

        result
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("couldn't read input from standard in");

    let args: Vec<String> = env::args().collect();

    let max_words: u32 = args[1].parse().expect("max_words argument could not be parsed as u32");

    let output: String = MarkovTable::new(&input).generate(max_words);

    println!("{}", output);
}

#[test]
fn test_parse() {
    let table = MarkovTable::parse("I like cake. I like pie.");

    assert_eq!(table.get(&("I", "like")), Some(&vec!["cake.", "pie."]));
    assert_eq!(table.get(&("like", "cake.")), Some(&vec!["I"]));
    assert_eq!(table.get(&("cake.", "I")), Some(&vec!["like"]));
}

#[test]
fn test_generate() {
    let mut mtable = MarkovTable::new("I like cake. I like pie");
    let result = mtable.seed([13, 84, 433, 33]).generate(6);

    assert_eq!(result, "I like cake. I like cake.");
}

#[test]
fn test_tokenize() {
    let tokens = MarkovTable::tokenize("I like cake.\n\n I like\tpie.");
    assert_eq!(tokens, vec!["I", "like", "cake.", "I", "like", "pie."]);
}
