use rand::random_range;
use std::{collections::HashMap, env, fs, path::Path};

type MarkovChain = HashMap<String, HashMap<char, i32>>;

const WORDLIST_FR_PATH: &str = "dataset/fr/words.txt";
const HELP_STRING: &str = "usage: mwg [-o order] [-w quantity]\n----------- options -----------\n-o order\tindicate the order of the markov chain\n-w quantity\tindicate the quantity of word to generate\n--help\t\tprint this help message";
const ORDER_FLAG: &str = "-o";
const QUANT_FLAG: &str = "-w";
const HELP_FLAG: &str = "--help";

struct Flags {
    order: usize,
    quantity: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if !(args.len() > 1) {
        println!("{HELP_STRING}");
        return;
    }

    let mut flags: Flags = Flags {
        order: 3,
        quantity: 5,
    };

    let mut i = 1;
    while i < 4 {
        if let Some(flag) = args.get(i) {
            match flag.as_str() {
                ORDER_FLAG => {
                    flags.order = args
                        .get(i + 1)
                        .expect(format!("\"order\" isn't specified\n{HELP_STRING}").as_str())
                        .parse::<usize>()
                        .expect(format!("\"order\" should be a number\n{HELP_STRING}").as_str());
                }
                QUANT_FLAG => {
                    flags.quantity = args
                        .get(i + 1)
                        .expect(format!("\"quantity\" isn't specified\n{HELP_STRING}").as_str())
                        .parse::<usize>()
                        .expect(format!("\"quantity\" should be a number\n{HELP_STRING}").as_str());
                }
                HELP_FLAG => println!("{HELP_STRING}"),
                _ => {
                    panic!("{flag} | Invalid argument\n{HELP_STRING}");
                }
            }
        }
        i += 2;
    }

    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(WORDLIST_FR_PATH)
        .into_string()
        .expect("Couldn't parse path into string");
    let txt = fs::read_to_string(path).expect("Couldn't open or read file");
    let chain = make_chain(&txt, flags.order);
    let generated_words = generate_words(&chain, flags.quantity);
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

    for _ in 0..quantity {
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
