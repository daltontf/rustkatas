use std::fs::File;
use std::io::{prelude::*, BufReader};
use regex::RegexSet;

#[macro_use]
extern crate lazy_static;

fn find_passports(buffer: &String) {
    if buffer.len() > 0 {
       lazy_static! {
            static ref RE_SET:RegexSet = RegexSet::new(&[
            r"byr:\S+",
            r"iyr:\S+",
            r"eyr:\S+",
            r"hgt:\S+",
            r"hcl:\S+",
            r"ecl:\S+",
            r"pid:\S+",
            //r"cid:\w+",
            ]).unwrap();
        }
        let matches: Vec<_> = RE_SET.matches(buffer).into_iter().collect();
        if matches.len() == RE_SET.len() {
            println!("Matches: {}", buffer.as_str());
        } else {
            println!("Failed: {}", buffer.as_str());
        }
    }
}

fn main() {
    let mut buffer = String::new();

    let args: Vec<String> = std::env::args().collect();
    let file = File::open(&args[1]).unwrap();

    for line in BufReader::new(file).lines() {
        let line_str = line.unwrap();
        if line_str.len() > 0 {
            buffer.push_str(line_str.as_str());
            buffer.push(' ');
        } else {
            find_passports(&buffer);
            buffer = String::new();
        }
    }
    find_passports(&buffer);
}
