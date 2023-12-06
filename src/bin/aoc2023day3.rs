use std::io; 

use ::bounded_vec_deque::BoundedVecDeque;

use regex::{Regex, Match};

use std::io::prelude::*;

fn has_adjacent_number(
    symbol_indices: &Vec<usize>,
    number_info: &(u32, usize, usize)   
) -> bool {
    for symbol_index in symbol_indices {
        if symbol_index + 1 >= number_info.1 && symbol_index <= &number_info.2 {
            return true;
        }
    }
    false
}

fn sum_adjacent_numbers(
    rows_symbol_indices: &BoundedVecDeque<Vec<usize>>,
    number_infos:&Vec<(u32, usize, usize)>) -> u32 {

    number_infos.iter().fold(0u32, |mut result, number_info| {
        let mut has_adjacency = false;
        for row_symbol_indices in rows_symbol_indices {
            has_adjacency |= has_adjacent_number(&row_symbol_indices, &number_info);
            if has_adjacency { break; } // not using symbols_in_rows..fold allows short-circuit
        }
        if has_adjacency {
            result += number_info.0;
            println!("{} has adjacency", number_info.0);
        }
        result
    })
}

fn main() -> io::Result<()> {
    let number_re = Regex::new(r"(\d+)").unwrap();
    let symbol_re = Regex::new(r"([^\d\.\s])").unwrap();

    let mut row_symbols: BoundedVecDeque<Vec<usize>> = BoundedVecDeque::new(3);
    // Behave like a row of no symbols is before first row
    let mut last_numbers_row: Vec<(u32, usize, usize)> = Vec::new();

    let mut result: u32 = 0;

    for line in io::stdin().lock().lines() {
        let line_str = &line.unwrap();

        row_symbols.push_back(
            if symbol_re.is_match(&line_str) {
                symbol_re.captures_iter(&line_str)
                    .map(|symbol_capture| {
                        let capture: Match = symbol_capture.get(0).unwrap(); 
                        capture.start()    
                    }).collect()
            } else {
                Vec::new() // no symbols on line
            });

        result += sum_adjacent_numbers(&row_symbols, &last_numbers_row);

        last_numbers_row = number_re.captures_iter(&line_str)
            .map(|number_capture| {
                let capture: Match = number_capture.get(0).unwrap(); 
                (capture.as_str().parse::<u32>().unwrap(),capture.start(), capture.end())    
            }).collect();
    }

    row_symbols.pop_front(); // remove third to last row symbol data and evaluate final row

    result += sum_adjacent_numbers(&row_symbols, &last_numbers_row);

    println!("Result = {}", result);

    Ok(())
}