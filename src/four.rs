use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

pub fn four_a() -> u32 {
    let mut total: u32 = 0;

    let path = Path::new("src/input_4.txt");
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), Error::description(&why)),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if is_passphrase_valid_a(line.unwrap()) { 
            total += 1; 
        }
    }

    return total;
}

pub fn four_b() -> u32 {
    return 0;
}

pub fn is_passphrase_valid_a(input: String) -> bool {
    let split = input.split(" ");

    let mut inputs: Vec<&str> = Vec::new();

    for s in split{        
        if is_string_in_list(s, &inputs) {
            return false;
        }

        inputs.push(s);
    }

    return true;
}

pub fn is_passphrase_valid_b(input: String) -> bool {
    let split = input.split(" ");

    let mut inputs: Vec<&str> = Vec::new();

    for s in split{        
        if is_anagram_in_list(s, &inputs) {
            return false;
        }

        inputs.push(s);
    }

    return true;
}

fn is_string_in_list(string: &str, strings: &[&str]) -> bool {
    for s in strings {
        if *s == string {
            return true;
        }
    }

    return false;
}

fn is_anagram_in_list(string: &str, strings: &[&str]) -> bool {
    for s in strings {
        if *s == string {
            //println!("S: {}, String: {}", s, string);
            return true;
        }

        if s.chars().count() == string.chars().count() {
            //println!("matching length!", s, string);
            for c in s.chars() {
                if !string_contains_char(string, c) {
                    continue;
                }
            }

            return true;
        }
    }

    return false;
}

fn string_contains_char(string: &str, c: char) -> bool {
    println!("{}", string);
    for s in string.chars() {
        if s == c {
            return true;
        }
    }

    return false;
}