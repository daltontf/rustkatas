use std::io;

use std::io::prelude::*;

use regex::Regex;


fn main() -> io::Result<()> {
    let line_re = Regex::new(r"^Game (\d+): (.*)$").unwrap();
    let color_reveal_re = Regex::new(r"(\d+) (.*)").unwrap();

    let mut result:u32 = 0;

    for line in io::stdin().lock().lines() {
        let line_str = &line.unwrap();
        
        //if line_re.is_match(line_str) {
            for cap in line_re.captures_iter(line_str) { 
                let game_no = cap[1].parse::<u32>().unwrap();
                let reveals = cap[2].split(';');
                let mut is_valid = true;
                for reveal in reveals {
                    //println!("reveal {}", reveal);
                    let reveal_colors = reveal.split(',');
                    for color_reveal in reveal_colors {
                        //println!("color_reveal {}", color_reveal);
                        //if color_reveal_re.is_match(color_reveal) {
                            for cap in color_reveal_re.captures_iter(color_reveal) { 
                                let amount = cap[1].parse::<u32>().unwrap();
                                let color = &cap[2];
                                //println!("amount, color = {},{}", amount, color);
                                is_valid &= match (color, amount) {
                                    ("red", amount) if amount < 13 => true,
                                    ("green", amount) if amount < 14 => true,
                                    ("blue", amount) if amount < 15 => true,
                                    _ => false                                    
                                }
                            }
                        //}
                    }
                }
                if is_valid {
                    println!("Game # {} is valid", game_no);
                    result += game_no;
                }
            }         
        //}        
    }

    println!("Result = {}", result);   

    Ok(())
}