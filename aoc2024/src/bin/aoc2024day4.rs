use std::fs::File;
use std::io::{prelude::*, BufReader};

fn findXmas(board: &Vec<Vec<char>>, row_index: usize, col_index:usize, delta_row:i8, delta_col:i8, last_char:char) -> usize {
    if (row_index == 0 && delta_row < 0) || (row_index == board.len() - 1 && delta_row > 0) {
        return 0
    }
    let new_row = (row_index as i16 + delta_row as i16) as usize;
    let line = &board[new_row];
    if (col_index == 0 && delta_col < 0) || (col_index == line.len() - 1 && delta_col > 0) {
        return 0
    }
    let new_col = (col_index as i16 + delta_col as i16) as usize;
    let new_char = line[new_col];

    match (last_char, new_char) {
        ('X', 'M') => findXmas(board, new_row, new_col, delta_row, delta_col, 'M'),
        ('M', 'A') => findXmas(board, new_row, new_col, delta_row, delta_col, 'A'),
        ('A', 'S') => 1,
        _ => 0
    }
}

fn findSamOrMas(board: &Vec<Vec<char>>, row_index: usize, col_index:usize, delta_row:i8, delta_col:i8, last_char:char) -> bool {
    if (row_index == 0 && delta_row < 0) || (row_index == board.len() - 1 && delta_row > 0) {
        return false
    }
    let new_row = (row_index as i16 + delta_row as i16) as usize;
    let line = &board[new_row];
    if (col_index == 0 && delta_col < 0) || (col_index == line.len() - 1 && delta_col > 0) {
        return false
    }
    let new_col = (col_index as i16 + delta_col as i16) as usize;
    let new_char = line[new_col];

    match (last_char, new_char) {
        ('A', 'S') => findSamOrMas(board, row_index, col_index, delta_row * -1, delta_col * -1, 'S'),
        ('A', 'M') => findSamOrMas(board, row_index, col_index, delta_row * -1, delta_col * -1, 'M'),
        ('S', 'M') | ('M', 'S') => true,
        _ => false
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();

    let board = BufReader::new(file)
        .lines().map(|line| line.unwrap().as_str().trim()
            .chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut result1:usize = 0;   
    let mut result2:usize = 0;   

    for (row_index, line) in board.iter().enumerate() {
        for (col_index, letter) in line.iter().enumerate() {
            if board[row_index][col_index] == 'X' {
                result1 += 
                    findXmas(&board, row_index, col_index,  0, -1, 'X') +
                    findXmas(&board, row_index, col_index,  1, -1, 'X') +
                    findXmas(&board, row_index, col_index,  1,  0, 'X') +
                    findXmas(&board, row_index, col_index,  1,  1, 'X') +
                    findXmas(&board, row_index, col_index,  0,  1, 'X') +
                    findXmas(&board, row_index, col_index, -1,  1, 'X') +
                    findXmas(&board, row_index, col_index, -1,  0, 'X') +
                    findXmas(&board, row_index, col_index, -1, -1, 'X');
            }
            if board[row_index][col_index] == 'A' {
                result2 += 
                    if findSamOrMas(&board, row_index, col_index,  -1, -1, 'A') &&
                       findSamOrMas(&board, row_index, col_index,  -1,  1, 'A') { 1 } else { 0 }
            }
        }
    }

    println!("Result1 = {}", result1);
    println!("Result2 = {}", result2);
}