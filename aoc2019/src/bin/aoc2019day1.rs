use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();

    let part1_result: i32 = BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap().parse::<i32>().unwrap() 
        }).fold(0, |sum, value| sum + value / 3 - 2);

        println!("part1_result = {}", part1_result);

        let file = File::open(&args[1]).unwrap();

    let part2_result: i32 = BufReader::new(file)
        .lines()
        .map(|line| {
              line.unwrap().parse::<i32>().unwrap()
        }).fold(0, |sum, value| {
            let mut weight = value;
            let mut fuel = 0;
            loop {
                weight = weight / 3 - 2;
                if weight <= 0 {
                    break;
                }
                fuel += weight   
            }
            sum + fuel
        });
    
        println!("part2_result = {}", part2_result);    
}

