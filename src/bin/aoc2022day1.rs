use std::fs::File;
use std::io::{prelude::*, BufReader};

use std::cmp::max;
use std::cmp::Reverse;

struct State {
    current: u32,
    highest: u32,
}

struct State2 {
    current: u32,
    highest: Vec<u32>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();

    let part1_result: State = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().trim().to_string())
        .fold(State { current: 0, highest:0 }, |state, value| {
           if value.is_empty() {
              State { current :0, highest: max(state.current, state.highest) }
           } else {
              State { current : state.current + value.parse::<u32>().unwrap(), highest: state.highest }
           }
        });

    println!("part1_result = {}", part1_result.highest);

    let file = File::open(&args[1]).unwrap();

    let part2_result: State2 = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().trim().to_string())
        .fold(State2 { current: 0, highest:Vec::new() }, |mut state, value| {
           if value.is_empty() {
              state.highest.push(state.current);
              state.highest.sort_by_key(|w| Reverse(*w));
              while state.highest.len() > 3 {
                state.highest.pop();
              }
              state.current = 0;
              
           } else {
              state.current += value.parse::<u32>().unwrap();
           }
           state
        });

    println!("part2_result = {:?}", part2_result.highest.iter().sum::<u32>());
}