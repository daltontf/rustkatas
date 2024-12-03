use std::fs::File;
use std::io::{prelude::*, BufReader};

use regex::Regex;


fn main() {
    let line_re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();
    
    let part1_result = BufReader::new(file).lines()
        .map(|result| result.unwrap())
        .filter(|line| {

        if let Some(capture) = line_re.captures_at(&line, 0) {
            let first = (capture[1].parse::<u32>().unwrap(), capture[2].parse::<u32>().unwrap());
            let second = (capture[3].parse::<u32>().unwrap(), capture[4].parse::<u32>().unwrap());

            (first.0 >= second.0 && first.1 <= second.1)
            || (second.0 >= first.0 && second.1 <= first.1)
        } else {
            false
        }
        
    }).count();

    println!("part1_result = {}", part1_result);


    let file = File::open(&args[1]).unwrap();
    
    let part2_result = BufReader::new(file).lines()
        .map(|result| result.unwrap())
        .filter(|line| {

        if let Some(capture) = line_re.captures_at(&line, 0) {
            let first = (capture[1].parse::<u32>().unwrap(), capture[2].parse::<u32>().unwrap());
            let second = (capture[3].parse::<u32>().unwrap(), capture[4].parse::<u32>().unwrap());

               (first.0  >= second.0 && first.0  <= second.1)
            || (first.1  >= second.0 && first.1  <= second.1)
            || (second.0 >= first.0  && second.0 <= first.1)
            || (second.1 >= first.0  && second.1 <= first.1)
        } else {
            false
        }
        
    }).count();

    println!("part2_result = {}", part2_result);
}