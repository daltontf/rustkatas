use std::fs::File;
use std::io::{prelude::*, BufReader};

struct State {
    previous_value: Option<u32>,
    increases: u32,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();

    let part1_result: State = BufReader::new(file)
        .lines()
        .flat_map(|line| line.unwrap().parse::<u32>())
        .fold(State { previous_value: Option::None, increases: 0 }, |state, value| {
            State {
                previous_value: Some(value),
                increases: match state.previous_value {
                    Some(previous_value) if value > previous_value => state.increases + 1,
                    _ => state.increases
                }
            }
        });

    println!("part1_result = {}", part1_result.increases);

    let file = File::open(&args[1]).unwrap();

    let part2_result: State = BufReader::new(file)
        .lines()
        .flat_map(|line| line.unwrap().parse::<u32>())
        .collect::<Vec<u32>>()
        .windows(3)
        .fold(State { previous_value: Option::None, increases: 0 }, |state, values| {
            let value = values.iter().sum();
            State {
                previous_value: Some(value),
                increases: match state.previous_value {
                    Some(previous_value) if value > previous_value => state.increases + 1,
                    _ => state.increases
                }
            }
        });

    println!("part2_result = {}", part2_result.increases);

}