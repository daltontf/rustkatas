use std::fs::File;
use std::io::{prelude::*, BufReader};

struct DeltaState {
    sign: i32,
    safe: bool,
}

impl DeltaState {
    fn new(sign: i32, safe: bool) -> Self {
        DeltaState{ sign, safe }
    }
}

fn are_safe(numbers:&[i32]) -> bool {
    numbers.iter().collect::<Vec<&i32>>().windows(2)
      .fold(DeltaState::new(0, true), |state, window| {
        if state.safe {
            let delta = window[1] - window[0];
            let delta_abs = delta.abs();
            let delta_valid = 0 < delta_abs && delta_abs < 4;
            if state.sign == 0 {
                DeltaState::new(delta, delta_valid)
            } else if state.sign > 0 {
                DeltaState::new(state.sign, delta_valid && delta > 0)
            } else {
                DeltaState::new(state.sign, delta_valid && delta < 0 )
            }
        } else {
            state
        }      
    }).safe
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();

    let total_safe= BufReader::new(file)
        .lines()
        .fold(0i32, |sum, line| {
            let report = line.unwrap().split(" ").map(|str| {
                str.parse::<i32>().unwrap()
            }).collect::<Vec<i32>>();             

            let mut is_safe = are_safe(&report);
            if !is_safe {
                let mut can_be_made_safe = false;
                for i in 0..report.len() {
                    let mut removed_deltas = report.clone();
                    removed_deltas.remove(i);
                    can_be_made_safe |= are_safe(&removed_deltas);
                }
                is_safe = can_be_made_safe;
            }
            sum + if is_safe { 1 } else { 0 }
        });

    println!("Total safe = {}", total_safe);    
}