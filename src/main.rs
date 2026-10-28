use std::{collections::HashMap, fmt::Display, fs, path::Path};

type MarkovChain = HashMap<String, HashMap<char, u32>>;

const WORDLIST_FR_PATH: &str = "dataset/fr/words.txt";
const CHAIN_ORDER: usize = 2;

fn main() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(WORDLIST_FR_PATH)
        .into_string()
        .expect("Couldn't parse path into string");
    let txt = fs::read_to_string(path).expect("Couldn't open or read file");
    let chain = make_chain(&txt, CHAIN_ORDER);
    _print_chain(&chain);
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
            let outer_key = inner_vec[i..i + order]
                .iter()
                .fold(String::from(""), |acc, x| {
                    if acc != "" {
                        format!("{acc}:{x}")
                    } else {
                        x.to_string()
                    }
                });
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

fn _print_chain(chain: &MarkovChain) {
    for (out_k, out_v) in chain {
        println!("[{out_k}]");
        for (in_k, in_v) in out_v {
            println!("{in_k} => {in_v}");
        }
        println!("--------------------------");
    }
}
