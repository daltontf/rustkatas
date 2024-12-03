use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();

    let part1_result: i32 = BufReader::new(file)
        .lines()
        .flat_map(|line| line.unwrap().chars().collect::<Vec<_>>())
        .fold(0, |floor, chr| {
            if chr == '(' {
                floor + 1
            } else if chr == ')' {
                floor - 1
            } else {
                floor
            }
        });

        
        println!("part1_result = {}", part1_result);

        let mut part2_result: Option<usize> = Option::None;
        let mut floor: i32 = 0;

        let file = File::open(&args[1]).unwrap();

        for (index, chr) in BufReader::new(file)
            .lines()
            .flat_map(|line| line.unwrap().chars().collect::<Vec<_>>())
            .enumerate() {
                if chr == '(' {
                    floor += 1;
                } else if chr == ')' {
                    floor -= 1
                } 
                if floor == -1 {
                    part2_result = Some(index + 1);
                    break;
                }   
            }


        println!("part2_result = {}", part2_result.unwrap());
}