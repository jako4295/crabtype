use dict::{Dict, DictIface};
use rand::seq::SliceRandom;
use std::fs::read_to_string;
use std::str;

pub fn load_files_to_vec(dicts: Dict<bool>) -> Vec<char> {
    let mut char_vec: Vec<char> = vec![];
    for i in dicts {
        if i.val {
            let source_path: String = "resources/".to_owned();
            let file_type: &str = ".txt";
            let s: String = read_to_string(source_path + &i.key + file_type).unwrap();
            let v: Vec<char> = s.chars().collect();
            for c in v {
                char_vec.push(c);
            }
        }
    }
    char_vec
}

pub fn chose_random(char_vec: Vec<char>) -> char {
    let output_char: &char = char_vec.choose(&mut rand::thread_rng()).unwrap();
    output_char.to_owned()
}

// fn main() {
//     let mut dict: Dict<bool> = Dict::<bool>::new();
//     dict.add("letters".to_string(), true);
//     dict.add("cap_letters".to_string(), true);
//     dict.add("numbers".to_string(), true);
//     let char_vec: Vec<char> = load_files_to_vec(dict);
//     println!(
//         "{:?}",
//         char_vec
//             .iter()
//             .map(|x| x.to_string() + " ")
//             .collect::<String>()
//     );
//     let rand = chose_random(char_vec);
//     println!("{rand}");
// }
