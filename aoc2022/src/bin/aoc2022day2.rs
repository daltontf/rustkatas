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

    let file = File::open(&args[1]).unwrap();

    let part2_result: i32 = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .fold(0, |total, chr| {
            let mut split = chr.split_whitespace();
            let game = (split.next().unwrap(), split.next().unwrap());

            let resp = match game {
                ("A", "X") => ("A", "Z"),
                ("C", "X") => ("C", "Y"),
                ("A", "Y") => ("A", "X"),
                ("B", "Y") => ("B", "Y"),
                ("C", "Y") => ("C", "Z"),
                ("A", "Z") => ("A", "Y"),
                ("C", "Z") => ("C", "X"),
                (x, y) => (x, y)
            };

            let score = match resp {
                ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
                ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
                _                                    => 0
            };

            println!("score = {}", score);

            total + score + match resp.1 {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _   => 0
            }
    });

        
    println!("part2_result = {}", part2_result);
}