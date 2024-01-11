use std::fs::File;
use std::io::{prelude::*, BufReader};

use std::collections::HashMap;

struct State {
    santa_coords: (i32, i32),
    map: HashMap<(i32, i32), u32>
}

struct State2 {
    santa_coords: (i32, i32),
    robot_coords: (i32, i32),
    is_robot: bool,
    map: HashMap<(i32, i32), u32>
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();

    let mut map:HashMap<(i32, i32), u32>= HashMap::new();
    let santa_coords = (0, 0);
    map.insert(santa_coords, 1);

    let part1_result: State = BufReader::new(file)
        .lines()
        .flat_map(|line| line.unwrap().chars().collect::<Vec<_>>())
        .fold(State { santa_coords, map }, |mut state, chr| {
            let new_coords = if chr == '^' {
                (state.santa_coords.0, state.santa_coords.1 - 1)
            } else if chr == '>' {
                (state.santa_coords.0 + 1, state.santa_coords.1)
            } else if chr == '<' {
                (state.santa_coords.0 - 1, state.santa_coords.1)
            } else if chr == 'v' {
                (state.santa_coords.0, state.santa_coords.1 + 1)
            } else {
                state.santa_coords
            };
            state.santa_coords = new_coords;
            state.map.entry(new_coords)
                    .and_modify(|counter| *counter += 1).or_insert(1);
            state
        });

        
    println!("part1_result = {}", part1_result.map.len());

    let file = File::open(&args[1]).unwrap();

    let mut map:HashMap<(i32, i32), u32>= HashMap::new();
    let santa_coords = (0, 0);
    let robot_coords = (0, 0);
    map.insert(santa_coords, 2);

    let part2_result: State2 = BufReader::new(file)
        .lines()
        .flat_map(|line| line.unwrap().chars().collect::<Vec<_>>())
        .fold(State2 { santa_coords, robot_coords, is_robot: false, map }, |mut state, chr| {
            let coords = if state.is_robot { state.robot_coords } else { state.santa_coords };
            let new_coords = if chr == '^' {
                (coords.0, coords.1 - 1)
            } else if chr == '>' {
                (coords.0 + 1, coords.1)
            } else if chr == '<' {
                (coords.0 - 1, coords.1)
            } else if chr == 'v' {
                (coords.0, coords.1 + 1)
            } else {
                coords
            };

            state.map.entry(new_coords)
                .and_modify(|counter| *counter += 1).or_insert(1);

            if state.is_robot {
                state.robot_coords = new_coords;    
            } else {
                state.santa_coords = new_coords;    
            }

            state.is_robot = !state.is_robot;            

            state
        });
        
        println!("part2_result = {}", part2_result.map.len());
}
