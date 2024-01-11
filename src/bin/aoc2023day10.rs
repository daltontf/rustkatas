use std::io;

use std::cmp::max;

use std::io::prelude::*;

fn find_animal(board: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (row_index, row) in board.iter().enumerate() {
        for (col_index, _) in row.iter().enumerate() {
            if board[row_index][col_index] == 'S' {
                return Some((row_index, col_index));
            }  
        }
    }
    
    Option::None
}

fn mark_distances(location: &(usize, usize), distance: i32, board: &Vec<Vec<char>>, distances: &mut Vec<Vec<i32>>) {
    let location_distance = distances[location.0][location.1];   
    if location_distance == -1 || location_distance > distance {
        distances[location.0][location.1] = distance;
        let current_char = board[location.0][location.1];
        if location.0 > 0 { // North?
            let next_char = board[location.0 - 1][location.1];
            //println!("North {} -> {}", current_char, next_char);
            if (distance == 0 || "+|JL".contains(current_char)) && "+|F7".contains(next_char) {
                mark_distances(&(location.0 - 1,location.1), distance + 1, &board, distances);    
            }
        }
        if location.1 < board[location.0].len() - 1 { // East?
            let next_char = board[location.0][location.1 + 1];
            //println!("East {} -> {}", current_char, next_char);
            if (distance == 0 || "+-FL".contains(current_char)) && "+-J7".contains(next_char) {
                mark_distances(&(location.0,location.1 + 1), distance + 1, &board, distances);    
            }
        }
        if location.0 < board.len() - 1 { // South?
            let next_char = board[location.0 + 1][location.1];
            //println!("South {} -> {}", current_char, next_char);
            if (distance == 0 || "+|F7".contains(current_char)) && "+|JL".contains(next_char) {
                mark_distances(&(location.0 + 1,location.1), distance + 1, &board, distances);    
            }
        }
        if location.1 > 0 { // West?
            let next_char = board[location.0][location.1 - 1];
            //println!("West {} -> {}", current_char, next_char);
            if (distance == 0 || "+-J7".contains(current_char)) && "+-FL".contains(next_char) {
                mark_distances(&(location.0,location.1 - 1), distance + 1, &board, distances);    
            }
        }
    }
}


fn main() {
    let board:Vec<Vec<char>> = io::stdin().lock().lines().map(|line| {
        line.unwrap().chars().collect()
    }).collect();

    let mut distances:Vec<Vec<i32>> = vec![vec![-1;board[0].len()]; board.len()];
    
    let animal_coords = find_animal(&board).unwrap();
    
    //println!("animal @ {:?}", animal_coords);
    
    mark_distances(&animal_coords, 0, &board, &mut distances);

    let farthest = distances.iter().fold(-1, |highest, row| {
        max(highest, row.iter().fold(highest, |row_highest, distance| {
            max(row_highest, *distance)
        }))  
    });

    println!("Farthest = {}", farthest);

}