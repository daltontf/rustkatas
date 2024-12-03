use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();

    let entries:Vec<u32> = BufReader::new(file)
        .lines()
        .map(|it| it.unwrap().parse::<u32>().unwrap())
        .collect();        

    for (i, ientry) in entries.iter().enumerate() {
        for (j, jentry) in entries.iter().enumerate() {
            if i != j && ientry + jentry == 2020 {
                println!("part1_result = {}", ientry * jentry)
            }
        }
    }

    let file = File::open(&args[1]).unwrap();

    let entries:Vec<u32> = BufReader::new(file)
        .lines()
        .map(|it| it.unwrap().parse::<u32>().unwrap())
        .collect();        

    for (i, ientry) in entries.iter().enumerate() {
        for (j, jentry) in entries.iter().enumerate() {
            for (k, kentry) in entries.iter().enumerate() {
                if i != j && i != k && j != k && kentry +ientry + jentry == 2020 {
                    println!("part2_result = {}", kentry * ientry * jentry)
                }
            }
        }
    }
}
