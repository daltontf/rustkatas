use std::io;

use std::io::prelude::*;

use regex::Regex;


fn main() -> io::Result<()> {
    let re1 = Regex::new(r"^.*?(\d{1}).*?$").unwrap();
    let re2 = Regex::new(r"^.*?(\d{1}).*(\d{1}).*?$").unwrap();

    let mut i: u32 = 0;

    for line in io::stdin().lock().lines() {
        let line_str = &line.unwrap();
        if re2.is_match(line_str) {
            for cap in re2.captures_iter(line_str) {
                i += cap[1].parse::<u32>().unwrap() * 10;
                i += cap[2].parse::<u32>().unwrap();
            }
        } else if re1.is_match(line_str) {
            for cap in re1.captures_iter(line_str) {
                i += cap[1].parse::<u32>().unwrap() * 10;
                i += cap[1].parse::<u32>().unwrap();
            }  
        }        
    }

    println!("Result = {}", i);   

    Ok(())
}