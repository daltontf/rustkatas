use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct State {
    last_value: Option<u8>,
    current_run: usize,
    sum: usize
}

impl State {
    fn next(self, value: &u8) -> State {
        if self.last_value.is_some() && *value == self.last_value.unwrap() {
            State { 
                last_value: Some(*value),
                current_run: self.current_run + *value as usize,
                sum: self.sum
            }
           } else if self.current_run > 0 {
            State { 
                last_value: Some(*value), 
                current_run: 0, 
                sum: self.sum + self.current_run 
            }
           } else {
            State { 
                last_value: Some(*value), 
                current_run: 0,
                sum: self.sum
            }
        }
    }

    fn end(self) -> State {
        State { 
            last_value: self.last_value, 
            current_run: 0, 
            sum: self.sum + self.current_run 
        }    
    }
}


fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();

    let digits: Vec<u8> = BufReader::new(file)
        .lines()
        .flat_map(|line| line.unwrap().chars().collect::<Vec<_>>())
        .map(|chr| chr as u8 - '0' as u8).collect();

    let part1_result = digits.iter()
        .fold(State { last_value: Option::None, current_run: 0, sum: 0 }, State::next);    

    println!("result = {:?}", part1_result.next(&digits[0]).end( ));

    let file = File::open(&args[1]).unwrap();

    let digits: Vec<u8> = BufReader::new(file)
        .lines()
        .flat_map(|line| line.unwrap().chars().collect::<Vec<_>>())
        .map(|chr| chr as u8 - '0' as u8).collect();

    let look_ahead = digits.len() / 2;

    let part2_result:u32 = digits.iter().enumerate()
        .fold(0, |sum, (index, value)| {
            if index < look_ahead {
                if *value == digits[index + look_ahead] {
                    return sum + *value as u32 * 2;
                }
            }
            sum
        });    

    println!("result = {:?}", part2_result);
}
