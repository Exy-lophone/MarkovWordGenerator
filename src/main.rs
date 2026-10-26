use std::{collections::HashMap, fs, path::Path};

const WORDLIST_FR_PATH: &str = "dataset/fr/words.txt";

fn main() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(WORDLIST_FR_PATH)
        .into_string()
        .expect("Couldn't parse path into string");
    let txt = fs::read_to_string(path).expect("Couldn't open or read file");
    // let bytes = txt.as_bytes();
    // for byte in bytes {
    //     if *byte == 10 {
    //         println!("LINEBREAK");
    //         println!("-----------");
    //         continue;
    //     }
    //     println!("{byte}")
    // }
    let data: Vec<Vec<char>> = txt
        .split("\r\n")
        .map(|x| format!("###{}$", x.to_lowercase()))
        .map(|x| x.chars().collect())
        .collect();
    // for word in data.iter() {
    //     for (i, c) in word.iter().enumerate() {
    //         println!("{i}: {c}");
    //     }
    //     println!("------------------------")
    // }
    let mut chain: HashMap<String, HashMap<char, u32>> = HashMap::new();
    for cs in data {
        for i in 0..cs.len() - 3 {
            let key = format!("{}:{}:{}", cs[i], cs[i + 1], cs[i + 2]);
            if let Some(chars_count) = chain.get_mut(&key) {
                if let Some(count) = chars_count.get_mut(&cs[i + 3]) {
                    *count += 1;
                } else {
                    chars_count.insert(cs[i + 3], 1);
                }
            } else {
                chain.insert(key, HashMap::from([(cs[i + 3], 1)]));
            }
        }
    }
    for (state_a, inner) in chain {
        println!("[{state_a}]");
        for (state_b, count) in inner {
            println!("{state_b} => {count}");
        }
        println!("--------------------------");
    }
}
