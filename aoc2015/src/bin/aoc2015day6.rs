use std::fs::File;
use std::io::{prelude::*, BufReader};

use regex::Regex;

enum Oper {
    Toggle,
    TurnOn, 
    TurnOff,
    NoOp
}

fn main() {
    let line_re = Regex::new(r"(toggle|turn off|turn on) (\d+),(\d+) through (\d+),(\d+)").unwrap();

    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();
   
    let matrix: Vec<Vec<bool>> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .fold( vec![vec![false; 1000]; 1000], |mut mat, line| {
            for cap in line_re.captures_iter(&line) { 
                let open_txt = cap[1].trim();
                let oper = match open_txt {
                    "turn on" => Oper::TurnOn,
                    "turn off" => Oper::TurnOff,
                    "toggle" => Oper::Toggle,
                    _ => Oper::NoOp
                };
                let from_x:usize = cap[2].parse().unwrap();
                let from_y:usize = cap[3].parse().unwrap();
                let to_x:usize = cap[4].parse().unwrap();
                let to_y:usize = cap[5].parse().unwrap();
                
                for x in from_x..=to_x {
                    for y in from_y..=to_y {
                        mat[y][x] = match oper {
                            Oper::TurnOff => false,
                            Oper::TurnOn => true,
                            Oper::Toggle => !mat[y][x],
                            Oper::NoOp => mat[y][x]
                        }    
                    }
                }
            }          

            mat
        });

    let part1_result = matrix.into_iter()
        .flat_map(|bits| bits)
        .fold(0, |sum, value: bool| if value { sum + 1 } else { sum });

    println!("part1_result = {}", part1_result);

    let file = File::open(&args[1]).unwrap();
    
    let matrix: Vec<Vec<i32>> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .fold( vec![vec![0; 1000]; 1000], |mut mat, line| {
            for cap in line_re.captures_iter(&line) { 
                let open_txt = cap[1].trim();
                let oper = match open_txt {
                    "turn on" => Oper::TurnOn,
                    "turn off" => Oper::TurnOff,
                    "toggle" => Oper::Toggle,
                    _ => Oper::NoOp
                };
                let from_x:usize = cap[2].parse().unwrap();
                let from_y:usize = cap[3].parse().unwrap();
                let to_x:usize = cap[4].parse().unwrap();
                let to_y:usize = cap[5].parse().unwrap();
                
                for x in from_x..=to_x {
                    for y in from_y..=to_y {
                        mat[y][x] += match oper {
                            Oper::TurnOff => -1,
                            Oper::TurnOn => 1,
                            Oper::Toggle => 2,
                            Oper::NoOp => 0
                        };
                        if mat[y][x] < 0 {
                            mat[y][x] = 0;
                        }   
                    }
                }

                
            }          

            mat
        });

    let part2_result = matrix.into_iter()
        .flat_map(|bits| bits)
        .fold(0, |sum, value| sum + value);

    println!("part2_result = {}", part2_result);
}