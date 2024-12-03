struct State {
    last: Option<char>,
    count: usize,
    result: Vec<char>
}

impl State {
    fn new() -> State {
        State {
            last: Option::None,
            count: 0,
            result: Vec::new()
        }
    }
}

fn apply(input : Vec<char>) -> Vec<char> {
    let mut state = input.iter().fold(State::new(), |mut state, chr| {
        if let Some(last) = state.last {
            if last == *chr {
                state.count += 1;
            } else {
                state.result.push(char::from_digit((state.count + 1) as u32, 10).unwrap());
                state.result.push(last);
                state.last = Some(*chr);
                state.count = 0;
            }
        } else {
            state.last = Some(*chr);    
        }
        state
    });
    if let Some(last) = state.last {
        state.result.push(char::from_digit((state.count + 1) as u32, 10).unwrap());
        state.result.push(last);    
    }

    state.result
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut result:Vec<char> = args[1].chars().collect();

    for _ in 0..50 {
        result = apply(result);
        //println!("{:?}", result);
    }

    println!("length = {}", result.len());
}