use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();

    let part1_result: i32 = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .fold(0, |total, chr| {
            let mut split = chr.split_whitespace();
            let game = (split.next().unwrap(), split.next().unwrap());
            let score = match game {
                ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
                ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
                _                                    => 0
            };
            total + score + match game.1 {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _   => 0
            }
    });

        
    println!("part1_result = {}", part1_result);
}