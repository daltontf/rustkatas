use std::fs::File;
use std::io::{prelude::*, BufReader};

use std::collections::HashSet;

fn find_route(
    board: &Vec<Vec<usize>>,
    past_coords: &mut Vec<(usize, usize)>,
    row_index: usize,
    col_index: usize,
    found1: &mut HashSet<(usize, usize)>,
    found2: &mut Vec<Vec<(usize, usize)>>
) {
    if board[row_index][col_index] == past_coords.len() {
        past_coords.push((row_index, col_index));

        if past_coords.len() == 10 {
            println!("found {:?}", past_coords);
            found1.insert((row_index, col_index));
            found2.push(past_coords.clone());
        } else {
            if row_index > 0 {
                find_route(board, past_coords, row_index - 1, col_index, found1, found2)
            }
            if row_index < board.len() - 1 {
                find_route(board, past_coords, row_index + 1, col_index, found1, found2)
            }

            let line = &board[row_index];

            if col_index > 0 {
                find_route(board, past_coords, row_index, col_index - 1, found1, found2)
            }
            if col_index < line.len() - 1 {
                find_route(board, past_coords, row_index, col_index + 1, found1, found2)
            }
        }
        past_coords.pop();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();

    let board = BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap().as_str().chars()
                .map(|c| { c.to_digit(10).unwrap() as usize })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut result1:usize = 0;
    let mut result2:usize = 0;
   
    for row_index in 0..board.len() {
        for col_index in 0..board[row_index].len() {
            let mut found1 = HashSet::<(usize, usize)>::new();
            let mut found2 = Vec::<Vec<(usize, usize)>>::new();

            if board[row_index][col_index] == 0 {
                find_route(&board, &mut Vec::new(), row_index, col_index, &mut found1, &mut found2);
            }
            result1 += found1.len();
            result2 += found2.len();
        }
    }

    println!("Result1 = {}", result1);
    println!("Result2 = {}", result2);
}
