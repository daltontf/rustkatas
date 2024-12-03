use std::fs::File;
use std::io::{prelude::*, BufReader};

use regex::Regex;

fn sort_captures(capture: &str) -> Vec<u32> {
    let mut result:Vec<u32> = capture.split_ascii_whitespace().map(|it| it.parse::<u32>().unwrap() ).collect();
    result.sort();
    result
}

fn main() {
    let line_re = Regex::new(r"^Card\s+\d+:\s+(.*)\s+\|\s+(.*)\s*$").unwrap();

    let mut sum_total = 0u32;

    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();
    
    for line in BufReader::new(file).lines() {
        let line_str = &line.unwrap();

         for capture in line_re.captures_iter(line_str) { 
            let sorted_captures1 = sort_captures(&capture[1]);
            let sorted_captures2 = sort_captures(&capture[2]);
            //println!("Captures {:?} | {:?}", sorted_captures1, sorted_captures2);
        
            let mut iter1 = sorted_captures1.iter();
            let mut iter2 = sorted_captures2.iter();
            let mut match_count = 0;

            let mut maybe1 = iter1.next();
            let mut maybe2 = iter2.next();
            
            loop {
                if let Some(value1) = maybe1 {
                    if let Some(value2) = maybe2 {
                        if value1 == value2 {
                            match_count += 1;
                            maybe1 = iter1.next();
                        } else if value1 < value2 {
                            maybe1 = iter1.next();
                        } else {
                            maybe2 = iter2.next();
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            //println!("match count = {}", match_count);
            sum_total += if match_count == 0 { 0 } else { 2u32.pow(match_count - 1) };
        }

    }
    println!("sum_total = {}", sum_total);
}