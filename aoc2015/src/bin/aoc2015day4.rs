use md5::{Md5, Digest};

fn find_first_match(leading: &str) -> Option<u32> {
    let mut first_match: Option<u32> = Option::None;
    
    for i in 1..std::u32::MAX {
        let mut hasher = Md5::new();
        hasher.update(format!("bgvyzdsv{}", i));

        let result = format!("{:X}", hasher.finalize());

        if result.starts_with(leading) {
            println!("{}", result);
            first_match = Some(i);
            break;   
        }
    }
    first_match
}

fn main() {     

    println!("part1_result = {}", find_first_match("00000").unwrap());
    println!("part2_result = {}", find_first_match("000000").unwrap());

}