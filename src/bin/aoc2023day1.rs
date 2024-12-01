use std::fs::File;
use std::io::{prelude::*, BufReader};

use regex::Regex;

fn letter_to_digit(letter: &str) -> u32 {
    match letter {
        "one"   => 1,
        "two"   => 2,
        "three" => 3,
        "four"  => 4,
        "five"  => 5,
        "six"   => 6,
        "seven" => 7,
        "eight" => 8,
        "nine"  => 9,
        _ => 0
    }
}

fn main() {
    let re1 = Regex::new(r"^.*?(\d{1}).*?$").unwrap();
    let re2 = Regex::new(r"^.*?(\d{1}).*(\d{1}).*?$").unwrap();

    let mut part1_result: u32 = 0;

    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();
    
    for line in BufReader::new(file).lines() {
        let line_str = &line.unwrap();
        if re2.is_match(line_str) {
            for cap in re2.captures_iter(line_str) {
                part1_result += cap[1].parse::<u32>().unwrap() * 10;
                part1_result += cap[2].parse::<u32>().unwrap();
            }
        } else if re1.is_match(line_str) {
            for cap in re1.captures_iter(line_str) {
                part1_result += cap[1].parse::<u32>().unwrap() * 10;
                part1_result += cap[1].parse::<u32>().unwrap();
            }  
        }        
    }

    println!("part1_result = {}", part1_result);  

    let re1 = Regex::new(r"^.*?(\d{1}|one|two|three|four|five|six|seven|eight|nine).*?$").unwrap();
    let re2 = Regex::new(r"^.*?(\d{1}|one|two|three|four|five|six|seven|eight|nine).*(\d{1}|one|two|three|four|five|six|seven|eight|nine).*?$").unwrap();

    let file = File::open(&args[1]).unwrap();
    
    let part2_result = BufReader::new(file)
        .lines()
        .fold(0,|total, line| {            

        let line_str = &line.unwrap();

        total + if let Some(cap) = re2.captures_at(line_str, 0) {
              cap[1].parse::<u32>().unwrap_or(letter_to_digit(&cap[1])) * 10
            + cap[2].parse::<u32>().unwrap_or(letter_to_digit(&cap[2]))
        } else if let Some(cap) = re1.captures_at(line_str, 0) {
            let digit = cap[1].parse::<u32>().unwrap_or(letter_to_digit(&cap[1]));
            digit * 10 + digit
        } else {
            0
        }  
    });

    println!("part2_result = {}", part2_result); 

}