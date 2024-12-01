use std::fs::File;
use std::io::{prelude::*, BufReader};

use regex::Regex;

fn char_count(input: &str, target: &char) -> usize {
    input.chars().into_iter().filter(|c| c == target).count()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();

    let file = File::open(&args[1]).unwrap();

    let mut total_valid = 0u32;

    for line in BufReader::new(file).lines() {
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
    println!("Part 1 result = {}", total_valid);

    let file = File::open(&args[1]).unwrap();

    let part2_result: i32 = BufReader::new(file)
        .lines()
        .map(|result| result.unwrap())
        .fold(0, |total, line| {
            if let Some(cap) = re.captures_at(&line, 0) {
                let pos1: usize = cap[1].parse().unwrap();
                let pos2: usize = cap[2].parse().unwrap();
                let chr = &cap[3].chars().next().unwrap();
                let password:Vec<char> = cap[4].chars().collect();

                let pos1_equal = password[pos1 - 1] == *chr;
                let pos2_equal = password[pos2 - 1] == *chr;

                if (pos1_equal || pos2_equal) && !(pos1_equal && pos2_equal) {
                    return total + 1;
                }
            }
            total  
        });

    println!("Part 2 result = {}", part2_result);

}
