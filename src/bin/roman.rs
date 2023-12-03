fn create_result(amt: i32, minus: i32, head: &str) -> String {
    let mut result = String::from(head);
    result.push_str(roman(amt - minus).as_str());
    result
}

fn roman(i: i32) -> String {
    if i >= 1000 {
        create_result(i, 1000, "M")
    } else if i >= 900 {
        create_result(i, 900, "CM")
    } else if i >= 500 {
        create_result(i, 500, "D")
    } else if i >= 400 {
        create_result(i, 400, "CD")
    } else if i >= 100 {
        create_result(i, 100, "C")
    } else if i >= 90 {
        create_result(i, 90, "XC")
    } else if i >= 50 {
        create_result(i, 50, "L")
    } else if i >= 40 {
        create_result(i, 40, "XL")
    } else if i >= 10 {
        create_result(i, 10, "X")
    } else if i >= 9 {
        create_result(i, 9, "IX")
    } else if i >= 5 {
        create_result(i, 5, "V")
    } else if i >= 4 {
        create_result(i, 4, "IV")
    } else if i >= 1 {
        create_result(i, 1, "I")
    } else {
        String::from("")
    }
}

fn main() {
    for x in 1..101 {
        println!("> {} = {}", x, roman(x));
    }
}
