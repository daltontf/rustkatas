struct State {
    is_valid: bool,
    double_found: Option<u8>, 
    previous: Option<u8>
}

impl State {
    fn new() -> State {
        State {
            is_valid: true,
            double_found: Option::None,
            previous: Option::None
        }
    }
}

fn main() {

    let part1_result = (123257..647015)
        .filter(|num| {
            let num_str = format!("{}", num);
            let bytes = num_str.as_bytes();
            let final_state = bytes.iter()
                .fold(State::new(), |mut state, curr| {
                    if let Some(previous) = state.previous {
                        if previous == *curr {
                            state.double_found = Some(previous);
                        }
                        state.is_valid &= previous <= *curr;
                    }
                    state.previous = Some(*curr);

                    state
                });
            final_state.double_found.is_some() && final_state.is_valid    
    }).count();

    println!("part1_result = {}", part1_result);     

    let part2_result = (123257..647015)
    .filter(|num| {
        let num_str = format!("{}", num);
        let bytes = num_str.as_bytes();
        let final_state = bytes.iter()
            .fold(State::new(), |mut state, curr| {
                if let Some(previous) = state.previous {
                    if previous == *curr {
                        if state.double_found.is_none() {
                            state.double_found = Some(previous);
                        }                       
                    }
                    state.is_valid &= previous <= *curr;
                }
                state.previous = Some(*curr);

                state
            });
        final_state.double_found.is_some() && final_state.is_valid    
    }).count();

println!("part2_result = {}", part2_result);  
}
