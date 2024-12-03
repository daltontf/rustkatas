use std::fs::File;
use std::io::{prelude::*, BufReader};

use std::cmp::max;

use regex::Regex;


fn main() {
    let line_re = Regex::new(r"^Game (\d+): (.*)$").unwrap();
    let color_reveal_re = Regex::new(r"(\d+) (.*)").unwrap();

    let mut part1_result:u32 = 0;

    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();
    
    for line in BufReader::new(file).lines() {
        let line_str = &line.unwrap();
        
            for cap in line_re.captures_iter(line_str) { 
                let game_no = cap[1].parse::<u32>().unwrap();
                let reveals = cap[2].split(';');
                let mut is_valid = true;
                for reveal in reveals {
                    let reveal_colors = reveal.split(',');
                    for color_reveal in reveal_colors {
                            for cap in color_reveal_re.captures_iter(color_reveal) { 
                                let amount = cap[1].parse::<u32>().unwrap();
                                let color = &cap[2];
                                is_valid &= match (color, amount) {
                                    ("red", amount) if amount < 13 => true,
                                    ("green", amount) if amount < 14 => true,
                                    ("blue", amount) if amount < 15 => true,
                                    _ => false                                    
                                }
                            }
                    }
                }
                if is_valid {
                    println!("Game # {} is valid", game_no);
                    part1_result += game_no;
                }
            }         
     
    }

    println!("part1_result = {}", part1_result);  

    let mut part2_result:u32 = 0;

    let file = File::open(&args[1]).unwrap();
    
    for line in BufReader::new(file).lines() {
        let line_str = &line.unwrap();
        
        for cap in line_re.captures_iter(line_str) { 
            let game_no = cap[1].parse::<u32>().unwrap();
            let reveals = cap[2].split(';');

            let mut maxes = (0, 0, 0);

            for reveal in reveals {
                let reveal_colors = reveal.split(',');
                for color_reveal in reveal_colors {
                    for cap in color_reveal_re.captures_iter(color_reveal) { 
                        let amount = cap[1].parse::<u32>().unwrap();
                        let color = &cap[2];
                        match (color, amount) {
                            ("red", amount) => maxes.0 = max(maxes.0, amount),
                            ("green", amount) => maxes.1 = max(maxes.1, amount),
                            ("blue", amount) => maxes.2 = max(maxes.2, amount),
                            _ => ()
                        }
                    }
                }                
            }
            part2_result += maxes.0 * maxes.1 * maxes.2;
        }               
    }

    println!("part2_result = {}", part2_result);    
}