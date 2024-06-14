use std::fs::read_to_string;
use std::io;
use std::str;
use dict::{ Dict, DictIface };
use rand::seq::SliceRandom;
use std::sync::Mutex;
use std::io::Read;
use std::io::Write;
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};

fn load_files_to_vec(dicts: Dict<bool>) -> Vec<char>{
    let mut char_vec: Vec<char> = vec![];
    for i in dicts {
        if i.val{
            let source_path: String = "src/".to_owned();
            let file_type: &str = ".txt";
            let s: String = read_to_string(source_path+&i.key+file_type).unwrap();
            let v: Vec<char> = s.chars().collect();
            for c in v{
                char_vec.push(c);
            }
        }
    }
    char_vec
}


fn play(char_vec: Vec<char>) {
    std::process::Command::new("clear").status().unwrap();
    let score: Mutex<i32> = Mutex::new(0);
    loop {
        let output_char: &char = char_vec.choose(&mut rand::thread_rng()).unwrap();
        println!("{output_char}");
        let stdin = 0; // couldn't get std::os::unix::io::FromRawFd to work 
        // on /dev/stdin or /dev/tty
        let termios = Termios::from_fd(stdin).unwrap();
        let mut new_termios = termios.clone();  // make a mutable copy of termios 
        // that we will modify
        new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
        tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
        let stdout = io::stdout();
        let mut reader = io::stdin();
        let mut buffer = [0;1];  // read exactly one byte
        // print!("Hit a key! ");
        stdout.lock().flush().unwrap();
        reader.read_exact(&mut buffer).unwrap();
        let s = str::from_utf8(&buffer).unwrap();
        let c: char = s.chars().next().expect("string is empty");
        if c == '\n' { 
            break;
        }
        std::process::Command::new("clear").status().unwrap();

        if &c == output_char{
            let mut num = score.lock().unwrap();
            *num += 1;
            println!("{num}");
            // println!("{}", score);
        }
        else{
            let mut num = score.lock().unwrap();
            *num -= 1;
            println!("{num}");

        }
        tcsetattr(stdin, TCSANOW, & termios).unwrap();  // reset the stdin to 

    }
                                                    // original termios data
}

fn main() {
    let mut dict: Dict<bool> = Dict::<bool>::new();
    dict.add("letters".to_string(), true);
    dict.add("cap_letters".to_string(), true);
    dict.add("numbers".to_string(), true);
    let char_vec: Vec<char> = load_files_to_vec(dict);
    play(char_vec);
    // println!("{:?}", char_vec.iter().map(|x| x.to_string() + " ").collect::<String>());
}