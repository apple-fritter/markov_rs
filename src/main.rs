use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn parse(string: &str) -> HashMap<(&str, &str), Vec<&str>> {
    let mut table: HashMap<(&str, &str), Vec<&str>> = HashMap::new();

    let words: Vec<&str> = string.split(" ").collect();
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

#[test]
fn it_works() {
    let table = parse("I like cake. I like pie.");

    assert_eq!(table.get(&("I", "like")), Some(&vec!["cake.", "pie."]));
    assert_eq!(table.get(&("like", "cake.")), Some(&vec!["I"]));
    assert_eq!(table.get(&("cake.", "I")), Some(&vec!["like"]));

   // assert_eq!(table.get(&("", "")), Some(&vec!["I"]));
   // assert_eq!(table.get(&("", "I")), Some(&vec!["like"]));
   // assert_eq!(table.get(&("I", "like")), Some(&vec!["pie.", "cake."]));
   // assert_eq!(table.get(&("like", "cake.")), Some(&vec!["I"]));
   // assert_eq!(table.get(&("cake.", "I")), Some(&vec!["like"]));

    //assert_eq!(table.get(&(None, None)).unwrap().unwrap(), vec!["I"]);
    //assert_eq!(table.get(&(None, Some("I"))).unwrap().unwrap(), vec!["like"]);
}
