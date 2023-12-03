use regex::Regex;

fn longest_run(input: &str, target: &char) -> usize {
    let mut longest_len: usize = 0;
    let mut latest_len: usize = 0;
    let mut running: bool = false;
    for chr in input.chars() {
        if chr == *target {
            if running {
                latest_len += 1;
            } else {
                latest_len = 1;
                running = true;
            }
        } else {
            if running && latest_len > longest_len {
                longest_len = latest_len;
            }
            running = false;
        }
    }
    if running && latest_len > longest_len {
        longest_len = latest_len;
    }
    longest_len
}

fn main() {
    let passwords = vec![
        "1-3 a: abcdeaaa",
        "1-3 b: cdefg", // fail
        "2-9 c: ccccccccc",
        "2-4 c: acdac",     // fail
        "1-3 a: caaaabced",  // fail
        "2-9 c: cccccccccc", // fail
    ];

    let re = Regex::new(r"^(\d{1})-(\d{1}) ([a-z]{1}): ([a-z]+)$").unwrap();

    for password_rule in passwords.iter() {
        if re.is_match(password_rule) {
            for cap in re.captures_iter(password_rule) {
                let min: usize = cap[1].parse().unwrap();
                let max: usize = cap[2].parse().unwrap();
                let chr = &cap[3].chars().next().unwrap();
                let password = &cap[4];
                let longest = longest_run(password, chr);
                if !(longest >= min && longest <= max) {
                    println!("Password '{}' does not match", password);
                }
            }
        } else {
            println!("Input invalid '{}'", password_rule);
        }
    }
}
