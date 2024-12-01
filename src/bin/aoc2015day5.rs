use std::fs::File;
use std::io::{prelude::*, BufReader};

use fancy_regex::Regex;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();

    let forbidden = vec!["ab", "cd", "pq", "xy"];
    let double_re = Regex::new(r"([a-z])\1{1}").unwrap();
    let vowels_re = Regex::new("[aeiou].*[aeiou].*[aeiou]").unwrap();

    let part1_result: usize = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| {
            !forbidden.iter().any(|it| line.contains(it))
          && double_re.is_match(line).unwrap()
          && vowels_re.is_match(line).unwrap()
        }).count();

    println!("part1_result = {}", part1_result);

    let file = File::open(&args[1]).unwrap();

    let repeated_two_re = Regex::new(r"([a-z][a-z]).*\1").unwrap();
    let one_between_two_re = Regex::new(r"([a-z]).\1").unwrap();

    let part2_result: usize = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| {
              repeated_two_re.is_match(line).unwrap()
          && one_between_two_re.is_match(line).unwrap()
        }).count();

    println!("part2_result = {}", part2_result);
}