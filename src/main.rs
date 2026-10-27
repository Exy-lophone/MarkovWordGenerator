use std::{collections::HashMap, fs, path::Path};

const WORDLIST_FR_PATH: &str = "dataset/fr/words.txt";
const CHAIN_ORDER: usize = 5;

fn main() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(WORDLIST_FR_PATH)
        .into_string()
        .expect("Couldn't parse path into string");
    let data = fs::read_to_string(path).expect("Couldn't open or read file");
    // print_bytes(&txt);
    let data = format_txt(&data, CHAIN_ORDER);
    // print_formatted_txt(&data);
    let chain = make_chain(&data);
    print_chain(&chain);
}

fn format_txt(s: &str, o: usize) -> Vec<Vec<char>> {
    s.split("\r\n")
        .map(|x| {
            format!("{}{x}$", vec!["#"; o].join(""))
                .to_lowercase()
                .chars()
                .collect()
        })
        .collect()
}

fn make_chain(outer_vec: &Vec<Vec<char>>) -> HashMap<String, HashMap<char, u32>> {
    if outer_vec.len() == 0 {
        panic!("Vector of formated data isn't populated");
    }
    let order = outer_vec[0]
        .iter()
        .fold(0, |acc, x| if *x == '#' { acc + 1 } else { acc });
    let mut chain: HashMap<String, HashMap<char, u32>> = HashMap::new();
    for inner_vec in outer_vec {
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

fn _print_bytes(s: &str) {
    for byte in s.as_bytes() {
        if *byte == 10 {
            println!("\\n");
            continue;
        }
        print!("{byte} ")
    }
}

fn _print_formatted_txt(v: &Vec<Vec<char>>) {
    for word in v.iter() {
        let s = word.iter().fold(String::from(""), |acc, x| {
            if acc != "" {
                format!("{acc}:{x}")
            } else {
                x.to_string()
            }
        });
        println!("{s}");
    }
}

fn print_chain(chain: &HashMap<String, HashMap<char, u32>>) {
    for (out_k, out_v) in chain {
        println!("[{out_k}]");
        for (in_k, in_v) in out_v {
            println!("{in_k} => {in_v}");
        }
        println!("--------------------------");
    }
}
