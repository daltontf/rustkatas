use std::fs::File;
use std::io::{prelude::*, BufReader};

use regex::Regex;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();

    let mut stacks:Vec<Vec<char>> = vec![];

    let move_re = Regex::new(r"^move (\d+) from (\d+) to (\d+)").unwrap();

    for line in BufReader::new(file).lines().map(Result::unwrap) {        
        if line.contains('[') { 
            let mut i = 0;
            let chars:Vec<char> = line.chars().collect();
            while i * 4 < line.len() {
                while stacks.len() <= i {
                    stacks.push(vec![]);
                }
                let chr = chars.get(i * 4 + 1).unwrap();
                if chr.is_ascii_alphabetic() {
                    let stack = stacks.get_mut(i).unwrap();
                    stack.insert(0, *chr);
                }

                i += 1;
            }
        } else if let Some(captures) = move_re.captures(&line) {
            let count:usize = captures[1].parse().unwrap();
            let from:usize = captures[2].parse().unwrap();
            let to:usize = captures[3].parse().unwrap();

            // Part - 1
            // for _ in 0..count {
            //     let chr = stacks.get_mut(from - 1).unwrap().pop().unwrap();
            //     stacks.get_mut(to - 1).unwrap().push(chr);
            // }
            // Part - 2
            
            let mut moving: Vec<char> = vec![];
            for _ in 0..count {
                 moving.push(stacks.get_mut(from - 1).unwrap().pop().unwrap());
            }
            for _ in 0..count {
                stacks.get_mut(to - 1).unwrap().push(moving.pop().unwrap());
            }
        }
    }

    let result: String = stacks.iter()
        .map(|stack| stack.last().unwrap_or(&' ')).collect();

    println!("result = {:?}", result)

}