use std::fs::File;
use std::io::{prelude::*, BufReader};
use regex::Regex;

fn main() {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();

    let mut buffer = String::new();

    BufReader::new(file).read_to_string(&mut buffer).unwrap();

    let result1 = re.captures_iter(&buffer).fold(0, |sum, capture| {
        sum + (capture[1].parse::<u32>().unwrap() * capture[2].parse::<u32>().unwrap())
    });

     println!("Result1 = {}", result1);

     let re2 = Regex::new(r"mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))").unwrap();

     let result2 = re2.captures_iter(&buffer).fold((0, true), |state, capture| {
        match capture[0].to_string().as_str() {
            "do()" => (state.0, true),
            "don't()" => (state.0, false),
            _ => if state.1 {
                    (state.0 + (capture[1].parse::<u32>().unwrap() * capture[2].parse::<u32>().unwrap()), state.1)
                 } else {
                    state
                 }
        }
    });

     println!("Result2 = {}", result2.0);
}