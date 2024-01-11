use std::fs::File;
use std::io::{prelude::*, BufReader};

use regex::Regex;

use std::cmp::min;


fn main() {
    let re = Regex::new(r"^(\d+)x(\d+)x(\d+).*$").unwrap();

    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();

    let part1_result: u32 = BufReader::new(file)
       .lines()
       .map(|line| {
          let line_str = line.unwrap();
          for cap in re.captures_iter(&line_str) {
            return Some(
                (cap[1].parse::<u32>().unwrap(),
                 cap[2].parse::<u32>().unwrap(),
                 cap[3].parse::<u32>().unwrap()));
            }
            Option::None
       })
       .fold(0u32, |sum, some_tuple| 
            sum + some_tuple.map(|tuple| 
                (2 * tuple.0 * tuple.1) +
                (2 * tuple.1 * tuple.2) +
                (2 * tuple.2 * tuple.0) + 
                min(min(tuple.0 * tuple.1, tuple.1 * tuple.2), tuple.2 * tuple.0)
            ).unwrap_or(0)
        );

    println!("part2_result = {}", part1_result);

    let file = File::open(&args[1]).unwrap();

    let part2_result: u32 = BufReader::new(file)
       .lines()
       .map(|line| {
          let line_str = line.unwrap();
          for cap in re.captures_iter(&line_str) {
            return Some(
                (cap[1].parse::<u32>().unwrap(),
                 cap[2].parse::<u32>().unwrap(),
                 cap[3].parse::<u32>().unwrap()));
            }
            Option::None
       })
       .fold(0u32, |sum, some_tuple| 
            sum + some_tuple.map(|tuple| 
                (tuple.0 * tuple.1 * tuple.2) +
                min(min(2 * (tuple.0 + tuple.1), 2 * (tuple.1 + tuple.2)), 2 * (tuple.2 + tuple.0))
            ).unwrap_or(0)
        );

    println!("part2_result = {}", part2_result)
}

   