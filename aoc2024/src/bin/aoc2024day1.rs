use std::fs::File;
use std::io::{prelude::*, BufReader};

use std::iter::zip;
use std::ops::Div;

use regex::Regex;

fn binary_search(nums:&Vec<u32>, target:u32) -> u32 {
    let (mut left, mut right) = (0 as usize, nums.len() - 1);
  
    // loop till the search space is exhausted
    while left <= right {
 
        // find the mid-value in the search space and compares it with the target
        let mid = (left + right).div(2);

        if target == nums[mid] {
            left = mid;
            while left > 0 && target == nums[left - 1] {
                left -= 1;
            }
            right = mid;
            while right + 1 < nums.len() && target == nums[right + 1] {
                right += 1;
            }
            break; 
        // if the target is less than the middle element, discard the right half
        } else if target < nums[mid] {
            if mid == 0 {
                return 0;
            }
            right = mid - 1
        } else {
            if mid == nums.len() {
                return 0;
            }
            left = mid + 1
        }    
    }
    return if right >= left {
        (right - left) as u32 + 1
    } else { 
        0
    }
}

fn main() {
    let re = Regex::new(r"^(\d+) +(\d+)").unwrap();

    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();

    let mut numbers = BufReader::new(file)
        .lines()
        .fold((Vec::new(), Vec::new()), |mut sum, line| {
            let line_str = line.unwrap();
            for cap in re.captures_iter(&line_str) {
                sum.0.push(cap[1].parse::<u32>().unwrap());
                sum.1.push(cap[2].parse::<u32>().unwrap());
            }
            sum
        });
    
    numbers.0.sort();
    numbers.1.sort();

    let result1:u32 = zip(&numbers.0, &numbers.1)
        .map(|tuple|  {
            tuple.0.abs_diff(*tuple.1)
        }).sum();

    println!("Result1 = {}", result1);

    let result2:u32 = numbers.0.iter().map(|number| {
        number * binary_search(&numbers.1, *number)
    }).sum();

    println!("Result2 = {}", result2);

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_it() {
        assert_eq!(3, binary_search(vec![2, 4, 5, 5, 5, 6, 6, 8, 9, 9, 9].as_ref(), 5));
        assert_eq!(0, binary_search(vec![2, 4, 5, 5, 5, 6, 6, 8, 9, 9, 9].as_ref(), 7));
        assert_eq!(1, binary_search(vec![2, 4, 5, 5, 5, 6, 6, 8, 9, 9, 9].as_ref(), 2));
        assert_eq!(3, binary_search(vec![2, 4, 5, 5, 5, 6, 6, 8, 9, 9, 9].as_ref(), 9));
        assert_eq!(0, binary_search(vec![2, 4, 5, 5, 5, 6, 6, 8, 9, 9, 9].as_ref(), 1));
        //assert_eq!(0, binary_search(vec![2, 4, 5, 5, 5, 6, 6, 8, 9, 9, 9].as_ref(), 10));
    }
}
