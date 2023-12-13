use std::io;

use std::io::prelude::*;

// TODO? Persistent Vectors

fn replace_at_index(input: &Vec<char>, index:usize, chr:char) -> Vec<char> {
    input.iter().enumerate().map(|(i, c)| if i == index { chr } else { *c }).collect()
}

fn expand_vectors(permutations:&mut Vec<Vec<char>>, input: &Vec<char>) {
    for (index, chr) in input.iter().enumerate() {
        if *chr == '?' {
            let with_hash = replace_at_index(input, index, '#');
            expand_vectors(permutations, &with_hash);
            let with_period = replace_at_index(input, index, '.');
            expand_vectors(permutations, &with_period);
            return
        }   
    }
    permutations.push(input.to_vec());
}

fn main() {
    let result:usize = io::stdin().lock().lines().map(|line| {
        let line_str = line.unwrap();

        let line_parts:Vec<&str> = line_str.split(' ').collect();

        let input:Vec<char> = line_parts.get(0).unwrap().to_string().chars().collect();
   
        let pattern:String = format!("^\\.*{}\\.*$", line_parts.get(1).unwrap().split(",")
            .map(|chr| format!("#{{{}}}", chr))
            .collect::<Vec<String>>().join("\\.+"));
   
        let pattern_re = regex::Regex::new(&pattern).unwrap();
       
        let mut permutations: Vec<Vec<char>> = Vec::new();
   
        expand_vectors(&mut permutations, &input);
   
        permutations.iter().filter(|p| {
            let pstring = p.into_iter().collect::<String>();
            pattern_re.is_match(&pstring)
        }).count()
    }).fold(0, |mut sum, value| { sum += value; sum });

    println!("result = {}", result);
}