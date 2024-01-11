use std::fs::File;
use std::io::{prelude::*, BufReader};

use std::collections::HashSet;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();

    let part1_result: i32 = BufReader::new(file)
        .lines()
        .flat_map(|line| line.unwrap().parse::<i32>())
        .fold(0, |sum, value| {
            sum + value
        });

    println!("part1_result = {}", part1_result);


    

    let mut part2_result: Option<i32> = Option::None;
    let mut frequencies: HashSet<i32> = HashSet::new();
    let mut freq = 0;
    frequencies.insert(freq);
    
    while part2_result.is_none() {
        let file = File::open(&args[1]).unwrap();

        for change in BufReader::new(file)
        .lines()
        .flat_map(|line| line.unwrap().parse::<i32>()).into_iter() {
            freq += change;
            if frequencies.contains(&freq) {
                part2_result = Some(freq);
            } else {
                frequencies.insert(freq);
            }
        }
    }
        

    println!("part2_result = {}", part2_result.unwrap());

}