use std::io;

use std::io::prelude::*;

use regex::Regex;

fn char_count(input: &str, target: &char) -> usize {
    input.chars().into_iter().filter(|c| c == target).count()
}

fn main() {
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();

    let mut total_valid = 0u32;

    for line in io::stdin().lock().lines() {
        let password_rule = &line.unwrap();

        if re.is_match(password_rule) {
            for cap in re.captures_iter(password_rule) {
                let min: usize = cap[1].parse().unwrap();
                let max: usize = cap[2].parse().unwrap();
                let chr = &cap[3].chars().next().unwrap();
                let password = &cap[4];
                let count = char_count(password, chr);
                if count >= min && count <= max {
                    total_valid += 1;
                } else {
                    println!("Failed: {}", password_rule);
                }
            }
        } else {
            println!("Input invalid '{}'", password_rule);
        }        
    }
    println!("Valid count = {}", total_valid);
}
