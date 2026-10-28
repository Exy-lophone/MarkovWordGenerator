use rand::random_range;
use std::{collections::HashMap, fs, path::Path};
type MarkovChain = HashMap<String, HashMap<char, i32>>;

const WORDLIST_FR_PATH: &str = "dataset/fr/words.txt";
const CHAIN_ORDER: usize = 4;

fn main() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(WORDLIST_FR_PATH)
        .into_string()
        .expect("Couldn't parse path into string");
    let txt = fs::read_to_string(path).expect("Couldn't open or read file");
    let chain = make_chain(&txt, CHAIN_ORDER);
    let generated_words = generate_words(&chain, 10);
    for word in generated_words {
        println!("Generated: {word}");
    }
}

fn make_chain(txt: &str, order: usize) -> MarkovChain {
    let order = if order == 0 { 1 } else { order };
    let data: Vec<Vec<char>> = txt
        .split("\r\n")
        .map(|x| {
            format!("{}{x}$", vec!["#"; order].join(""))
                .to_lowercase()
                .chars()
                .collect()
        })
        .collect();
    let mut chain: MarkovChain = HashMap::new();

    for inner_vec in data {
        for i in 0..inner_vec.len() - order {
            let outer_key: String = inner_vec[i..i + order].iter().collect();
            let inner_key = inner_vec[i + order];
            if let Some(inner_map) = chain.get_mut(&outer_key) {
                if let Some(count) = inner_map.get_mut(&inner_key) {
                    *count += 1;
                } else {
                    inner_map.insert(inner_key, 1);
                }
            } else {
                chain.insert(outer_key, HashMap::from([(inner_key, 1)]));
            }
        }
    }
    chain
}

fn generate_words(chain: &MarkovChain, quantity: usize) -> Vec<String> {
    let quantity = if quantity == 0 { 1 } else { quantity };
    let mut generated: Vec<String> = Vec::with_capacity(quantity);
    let order = match chain.keys().next() {
        Some(x) => {
            let count = x.chars().count();
            if count == 0 {
                panic!("Markov chain isn't properly formated !")
            }
            count
        }
        None => panic!("Markov chain is empty !"),
    };

    for i in 0..quantity {
        let mut word = vec!['#'; order];
        loop {
            let key: String = word[word.len() - order..].iter().collect();
            let value = match chain.get(&key) {
                Some(x) => x,
                None => {
                    word.push('$');
                    break;
                }
            };
            let (ranges, max) = get_ranges(value);
            let rnd = random_range(0..max);
            for range in ranges {
                if rnd >= range.1 && rnd <= range.2 {
                    word.push(range.0);
                    if range.0 == '$' {
                        break;
                    }
                }
            }
        }
        generated.push(
            word.iter()
                .filter(|x| !(**x == '$' || **x == '#'))
                .collect(),
        );
    }
    generated
}

fn get_ranges(hmap: &HashMap<char, i32>) -> (Vec<(char, i32, i32)>, i32) {
    let mut ranges = Vec::with_capacity(hmap.len());
    let mut tmp = -1;
    let mut sum = 0;
    for (k, v) in hmap {
        ranges.push((*k, tmp + 1, tmp + v));
        tmp += v;
        sum += v;
    }
    (ranges, sum)
}

fn _print_chain(chain: &MarkovChain) {
    for (out_k, out_v) in chain {
        println!("[{out_k}]");
        for (in_k, in_v) in out_v {
            println!("{in_k} => {in_v}");
        }
        println!("--------------------------");
    }
}
