use std::io;

use std::io::prelude::*;

use regex::Regex;
struct MapperEntry {
    destination_start: u64,
    source_start: u64,
    range: u64
}

impl MapperEntry {
    
    fn perform_mapping(&self, input: u64) -> u64 {
        if input >= self.source_start && input < self.source_start + self.range {
            self.destination_start + (input - self.source_start)
        } else {
            input
        }
    }
}

struct Mapper {
    from: String,
    to: String,
    entries: Vec<MapperEntry>
}

fn main() {
    let seed_re = Regex::new(r"^seeds:\s+(.*)$").unwrap();
    let map_re = Regex::new(r"^(.*)-to-(.*) map:").unwrap();
    let entry_re = Regex::new(r"(\d+)\s+(\d+)\s+(\d+)").unwrap();

    let mut seeds: Vec<u64> = Vec::new();
    let mut mappers: Vec<Mapper> = Vec::new();

    for line in io::stdin().lock().lines() {
        let line_str = &line.unwrap();

        for seed_capture in seed_re.captures_iter(&line_str) {
            seeds = seed_capture[1].split_whitespace()
                .map(|it| it.parse::<u64>().unwrap())
                .collect();
        }

        for mapper_capture in map_re.captures_iter(&line_str) {
            let mapper = Mapper {
                from: mapper_capture[1].to_string(),
                to: mapper_capture[2].to_string(),
                entries: Vec::new() 
            };
            // TODO maybe validate previous mapper.to = mapper.from
            mappers.push(mapper);
        }

        for entry_capture in entry_re.captures_iter(&line_str) {
            let mapper_entry = MapperEntry {
                destination_start: entry_capture[1].parse::<u64>().unwrap(),
                source_start: entry_capture[2].parse::<u64>().unwrap(),
                range: entry_capture[3].parse::<u64>().unwrap(),
            };
            if let Some(last_mapper) = mappers.last_mut() {
                last_mapper.entries.push(mapper_entry);
            }
        }
    }

    let lowest: Option<u64> = seeds.iter().fold(None, |lowest_result, seed| {
        let mut result = *seed;
        for mapper in mappers.iter() {
            for mapper_entry in mapper.entries.iter() {
                let next_result = mapper_entry.perform_mapping(result);
                if next_result != result {
                    result = next_result;
                    break;
                }
            }
        }

        match lowest_result {
            Some(value) if result > value => lowest_result,
            _ => Some(result)
        }
      });

      if let Some(lowest_value) = lowest {
        println!("Lowest = {}", lowest_value);
      }
}