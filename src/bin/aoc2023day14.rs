use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {                 

    let mut shifted:Vec<Vec<char>> = Vec::new();   

    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();

    for (row_index, row) in BufReader::new(file).lines().map(|row| row.unwrap() ).enumerate() {
        if row_index == 0 {
            shifted.push(row.chars().into_iter().collect());
        } else {
            let mut new_row:Vec<char> = Vec::new();
            for (column_index, chr) in row.chars().enumerate() { 
                let mut prev_row = row_index - 1;
                if chr == 'O' && shifted[prev_row][column_index] == '.' {
                    loop {
                        if prev_row == 0 || shifted[prev_row - 1][column_index] != '.' {
                            break;
                        } else {
                            prev_row -= 1;
                        }
                    } 
                    shifted[prev_row][column_index] = 'O';
                    new_row.push('.');
                } else {
                    new_row.push(chr);
                }
            }
            shifted.push(new_row);
        } 
    }
    
    let shifted_size = shifted.len();
    

    let result = shifted.iter().enumerate().fold(0, |sum, (index, row)| {
        let row_sum = row.iter().fold(sum, |row_sum, chr| {
            //println!("row_sum = {}", row_sum);
            if *chr == 'O' { 
                row_sum + (shifted_size - index)
            } else { 
                row_sum
            }    
        });
        row_sum
    });
    
    println!("result = {}", result);
}