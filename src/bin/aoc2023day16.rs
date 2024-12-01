use std::fs::File;
use std::io::{prelude::*, BufReader};

use std::cmp::max;

#[derive(PartialEq, Eq)]
struct Direction {
    pub delta_x: i32,
    pub delta_y: i32,
    pub bits:    u8
}

impl Direction {
    pub const NORTH: Direction = Direction{ delta_y: -1, delta_x:  0, bits: 0b0001 };
    pub const SOUTH: Direction = Direction{ delta_y:  1, delta_x:  0, bits: 0b0010 };
    pub const WEST: Direction =  Direction{ delta_y:  0, delta_x: -1, bits: 0b0100 };
    pub const EAST: Direction =  Direction{ delta_y:  0, delta_x:  1, bits: 0b1000 };
}

fn start_beam(x: usize, y:usize, direction:&Direction, board:&Vec<Vec<char>>, beam_track: &mut Vec<Vec<u8>>) {
    let mut new_x = x;
    let mut new_y = y;
    let mut new_direction = direction;

    loop {
        if beam_track[new_y][new_x] & new_direction.bits > 0 {
            break; // Detect loop using bitmap to see if visited in same direction
        } 
        beam_track[new_y][new_x] |= new_direction.bits;

        let tile =  board[new_y][new_x]; 

        match (tile, new_direction,) {
            ('/', &Direction::EAST) => new_direction = &Direction::NORTH,
            ('/', &Direction::WEST) => new_direction = &Direction::SOUTH,
            ('/', &Direction::NORTH) => new_direction = &Direction::EAST,
            ('/', &Direction::SOUTH) => new_direction = &Direction::WEST,
            ('\\', &Direction::EAST) => new_direction = &Direction::SOUTH,
            ('\\', &Direction::WEST) => new_direction = &Direction::NORTH,
            ('\\', &Direction::NORTH) => new_direction = &Direction::WEST,
            ('\\', &Direction::SOUTH) => new_direction = &Direction::EAST,
            ('|', &Direction::EAST) | ('|', &Direction::WEST) => {
                new_direction = &Direction::SOUTH;
                if new_y > 0 {
                    start_beam(new_x, new_y, &Direction::NORTH, board, beam_track);
                }
            },
            ('-', &Direction::NORTH) | ('-', &Direction::SOUTH) => {
                new_direction = &Direction::EAST;
                if new_x > 0 {
                    start_beam(new_x, new_y, &Direction::WEST, board, beam_track);
                }
            }
            _ => ()
        }
        
        if new_direction.delta_x == -1 && new_x < 1 
        || new_direction.delta_x ==  1 && new_x + 1 >= beam_track[new_y].len()
        || new_direction.delta_y == -1 && new_y < 1
        || new_direction.delta_y ==  1 && new_y + 1 >= beam_track.len() {
            break;
        }

        new_x = (new_x as i32 + new_direction.delta_x) as usize;
        new_y = (new_y as i32 + new_direction.delta_y) as usize;
    }
}

fn perform_beam(start_x: usize, start_y: usize, direction: &Direction, board:&Vec<Vec<char>>) -> u32 {
    let mut beam_track:Vec<Vec<u8>> = vec![vec![0;board[0].len()]; board.len()];
        
    start_beam(start_x, start_y, direction, &board,&mut beam_track);
    beam_track.iter().fold(0, |total, row| {
        row.iter().fold(total, |total, value| if *value > 0 { total + 1 } else {total})
    })   
}

fn part1(board:&Vec<Vec<char>>) {
    let result = perform_beam(0, 0, &Direction::EAST, board);

    println!("Part 1 Result = {}", result);
}

fn part2(board:&Vec<Vec<char>>) {
    let mut result = 0;

    for index in 0..board[0].len() {
        result = max(result,perform_beam(index, 0, &Direction::SOUTH, board));
        result = max(result,perform_beam(index, board.len() - 1, &Direction::NORTH, board));
    }

    for index in 0..board.len() {
        result = max(result, perform_beam(0, index, &Direction::EAST, board));
        result = max(result, perform_beam(board[0].len() -1 , index, &Direction::WEST, board));
    }
 
    println!("Part 2 Result = {}", result);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();

    let board:Vec<Vec<char>> = BufReader::new(file).lines().map(|line| {
        line.unwrap().chars().collect()
    }).collect();

    part1(&board);

    part2(&board);
}