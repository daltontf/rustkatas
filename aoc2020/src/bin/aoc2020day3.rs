use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();
    
    let pattern: Vec<String> = BufReader::new(file).lines().map(|it| it.unwrap()).collect();

    let deltax = 3;
    
    let mut x = 0;
    let mut y = 0;
    let mut tree_count = 0;
    let grid_width = pattern[0].len();

    while y < pattern.len() {
        let row_chars:Vec<char> = pattern[y].chars().collect();
        if row_chars[x] == '#' {
            tree_count += 1;
        } 
        y += 1;
        x = (x + deltax) % grid_width;
    }

    println!("trees hit = {}", tree_count);
}
