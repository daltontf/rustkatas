use std::fs::File;
use std::io::{prelude::*, BufReader};

// TODO? Persistent Vectors

fn replace_at_index(input: &Vec<char>, index:usize, chr:char) -> Vec<char> {
    input.iter().enumerate().map(|(i, c)| if i == index { chr } else { *c }).collect()
}

fn expand_vectors(match_count:&mut usize, input: &Vec<char>, start_index:usize, pattern_re:&regex::Regex) {
    for (index, chr) in input.iter().enumerate().skip(start_index) {
        if *chr == '?' {
            let with_hash = replace_at_index(input, index, '#');
            expand_vectors(match_count, &with_hash, index, pattern_re );
            let with_period = replace_at_index(input, index, '.');
            expand_vectors(match_count, &with_period, index, pattern_re);
            return
        }   
    }

    let input_str =  input.into_iter().collect::<String>();

    if pattern_re.is_match(&input_str) {
        *match_count += 1;
    }
}

fn multiply<T: Copy>(input: &Vec<T>, separator: T) -> Vec<T> {
    input.iter()
        .chain(Some(&separator).into_iter())
        .chain(input.iter())
        .chain(Some(&separator).into_iter())
        .chain(input.iter())
        .chain(Some(&separator).into_iter())
        .chain(input.iter())
        .chain(Some(&separator).into_iter())
        .chain(input.iter())
        .map(|chr| *chr).collect()
}

fn main() {
    let mut match_count: usize = 0;

    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();
    
    for line in BufReader::new(file).lines() {
        let line_str = line.unwrap();

        let line_parts:Vec<&str> = line_str.split(' ').collect();

        let input:Vec<char> = multiply(
            &mut line_parts.get(0).unwrap().to_string().chars().collect(),
            '?');

        println!("{:?}", input);    

        let pattern_str: String = line_parts.get(1).unwrap().to_string();

        let pattern = format!("{},{},{},{},{}", pattern_str, pattern_str, pattern_str, pattern_str, pattern_str); 
      
        println!("{}", pattern);     

        let pattern:String = format!("^\\.*{}\\.*$", pattern.split(",")
            .map(|chr| format!("#{{{}}}", chr))
            .collect::<Vec<String>>().join("\\.+"));
   
        let pattern_re = regex::Regex::new(&pattern).unwrap();         
   
        expand_vectors(&mut match_count, &input, 0, &pattern_re);
    }

    println!("result = {}", match_count);
}