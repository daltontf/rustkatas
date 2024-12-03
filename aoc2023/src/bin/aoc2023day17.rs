use std::fs::File;
use std::io::{prelude::*, BufReader};


#[derive(PartialEq, Eq)]
struct Direction  {
    pub delta_x: i32,
    pub delta_y: i32,
}

impl Direction {
    pub const NORTH: Direction = Direction{ delta_y: -1, delta_x:  0 };
    pub const SOUTH: Direction  = Direction{ delta_y:  1, delta_x:  0 };
    pub const WEST: Direction  =  Direction{ delta_y:  0, delta_x: -1, };
    pub const EAST: Direction  =  Direction{ delta_y:  0, delta_x:  1, };

    fn directions() -> Vec<&'static Direction> {
        vec![&Self::NORTH, &Self::EAST, &Self::SOUTH, &Self::WEST]
    }

    fn get_opposite(&self) -> &Direction {
        match self {
            &Direction::EAST => &Direction::WEST,
            &Direction::WEST => &Direction::EAST,
            &Direction::NORTH => &Direction::SOUTH,
            &Direction::SOUTH => &Direction::NORTH,
            _ => self
        }
    }
}

fn find_best_path(
    board: &Vec<Vec<u8>>,
    x:usize, y:usize,    
    last_direction:&Direction,
    dir_count:u8,
    heat_loss_map: &mut Vec<Vec<u32>>,
    mut accumulated_heat_loss: u32,
)  {
    let current_heat_loss = board[y][x] as u32;
    let lowest_map_value = heat_loss_map[y][x];
    accumulated_heat_loss += current_heat_loss;    

    

    if y + 1 == board.len() && x + 1 == board[y].len() {
        if lowest_map_value == 0 || (accumulated_heat_loss < lowest_map_value) {
            heat_loss_map[y][x] = accumulated_heat_loss;
            println!("{}", accumulated_heat_loss);           
        }
        return;
    } else {
        //if dir_count == 0 {
        if lowest_map_value == 0 || (accumulated_heat_loss < lowest_map_value) {
            heat_loss_map[y][x] = accumulated_heat_loss;
        } else if current_heat_loss > 0 {
            return;
        }
        //}
    }
    
    for direction in Direction::directions() {
        if direction.delta_x == -1 && x < 1 
        || direction.delta_x ==  1 && x + 1 >= board[y].len()
        || direction.delta_y == -1 && y < 1
        || direction.delta_y ==  1 && y + 1 >= board.len() {
            continue
        }
    
        let new_x = (x as i32 + direction.delta_x) as usize;
        let new_y = (y as i32 + direction.delta_y) as usize;
        let new_dir_count = if last_direction == direction { dir_count + 1 } else { 0 };

        if new_dir_count < 3 {
            find_best_path(&board, new_x, new_y, direction, 
                new_dir_count,
                heat_loss_map, accumulated_heat_loss);            
        }     
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();
    
    let board:Vec<Vec<u8>> =         
        BufReader::new(file).lines()     
        .map(|line| {
        line.unwrap().chars().into_iter().map(|chr| {
            chr.to_digit(10).unwrap() as u8
        }).collect()
    }).collect(); 

    let mut heat_loss_map:Vec<Vec<u32>> = vec![vec![0;board[0].len()]; board.len()];
   
    find_best_path(&board, 0, 0, &Direction::SOUTH, 0,
        &mut heat_loss_map, 0);
    
    for row in heat_loss_map {
        println!("{:?}", row);
    }
}