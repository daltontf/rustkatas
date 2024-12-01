use std::fs::File;
use std::io::{prelude::*, BufReader};

enum StateType {
    Unquote,
    Quote,
    Escaped,
    Hex,
    Hex2nd,
}

struct State {
    state_type: StateType,
    total_chars: usize,
    total_input: usize
 }

impl State {
    fn new() -> State {
        State {
            state_type: StateType::Unquote,
            total_chars: 0,
            total_input: 0
        }
    }

    fn next(mut self, input: char) -> State {
        self.total_input += 1;
        self.state_type = match (self.state_type, input) {
            (StateType::Unquote, '\"') => StateType::Quote,
            (StateType::Quote, '\\') => StateType::Escaped,
            (StateType::Quote, '\"') => StateType::Unquote,
            (StateType::Escaped, 'x') => StateType::Hex,
              (StateType::Quote, _)
            | (StateType::Escaped, _) 
            | (StateType::Hex2nd, _) => { self.total_chars += 1; StateType::Quote }
            (StateType::Hex, _) => StateType::Hex2nd,
            _ => StateType::Unquote
        };
        self
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();
   
    let state: State = BufReader::new(file)
       .lines()
        .flat_map(|line| line.unwrap().to_string().chars().collect::<Vec<char>>())
        .fold(State::new(), State::next);

    println!("part1_result = {}", state.total_input - state.total_chars);
}