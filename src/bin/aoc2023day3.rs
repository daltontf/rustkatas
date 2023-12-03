use std::{io, collections::VecDeque};

use regex::{Regex, Match};

use std::io::prelude::*;

fn has_number_adjacent(
    symbols: &Vec<usize>,
    number: &(u32, usize, usize)   
) -> bool {
    for symbol_index in symbols {
        println!("number {} {} to {}, symbol at {}", number.0, number.1, number.2, symbol_index);
        if symbol_index + 1 >= number.1 && symbol_index <= &number.2 {
            return true;
        }
    }
    false
}

fn sum_adjacent_numbers(
    row_symbols: &VecDeque<Vec<usize>>,
    numbers:&Vec<(u32, usize, usize)>) -> u32 {

    let mut result: u32 = 0;

    if !numbers.is_empty() {       
        for number in numbers {
            let mut has_adjacency = false;
            for symbols in row_symbols {
                has_adjacency |= has_number_adjacent(&symbols, &number);
            }
            if has_adjacency {
                println!("Number {} has adjacency", number.0);
                result += number.0;
            } else {
                println!("Number {} has NO adjacency", number.0);
            }
        }
    }

    result
}

fn main() -> io::Result<()> {
    let number_re = Regex::new(r"(\d+)").unwrap();
    let symbol_re = Regex::new(r"([\$#\+\*])").unwrap();

    let mut row_symbols: VecDeque<Vec<usize>> = VecDeque::new();
    let mut last_numbers_row: Vec<(u32, usize, usize)> = Vec::new();

    let mut result: u32 = 0;

    row_symbols.push_back(Vec::new()); // 

    for line in io::stdin().lock().lines() {
        let line_str = &line.unwrap();

        row_symbols.push_back(symbol_re.captures_iter(&line_str)
            .map(|symbol_capture| {
                let capture: Match = symbol_capture.get(0).unwrap(); 
                capture.start()    
            }).collect());

        if row_symbols.len() > 3 {
            row_symbols.pop_front();
        }

        result += sum_adjacent_numbers(&row_symbols, &last_numbers_row);

        last_numbers_row = number_re.captures_iter(&line_str)
            .map(|number_capture| {
                let capture: Match = number_capture.get(0).unwrap(); 
                (capture.as_str().parse::<u32>().unwrap(),capture.start(), capture.end())    
            }).collect();
    }

    row_symbols.push_back(Vec::new());
    if row_symbols.len() > 3 {
        row_symbols.pop_front();
    } 

    result += sum_adjacent_numbers(&row_symbols, &last_numbers_row);

    println!("Result = {}", result);

    Ok(())
}