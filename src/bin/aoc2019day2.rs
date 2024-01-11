use std::num::ParseIntError;

fn execute(code_as_strings: Vec<&str>) -> Result<Vec<i32>, &str> {
    let code_result: Result<Vec<i32>, ParseIntError> =
        code_as_strings.into_iter().map(|s| s.parse()).collect();

    if code_result.is_ok() {
        let mut pc = 0;

        let mut code = code_result.unwrap();

        while code[pc] != 99 {
            if code[pc] == 1 {
                let target = code[pc + 3] as usize;
                code[target] = code[code[pc + 1] as usize] + code[code[pc + 2] as usize];
                pc += 4;
            } else if code[pc] == 2 {
                let target = code[pc + 3] as usize;
                code[target] = code[code[pc + 1] as usize] * code[code[pc + 2] as usize];
                pc += 4;
            } else {
                pc += 1;
            }
        }
        Result::Ok(code)
    } else {
        code_result.map_err(|_e| "parse error")
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();

    let result = execute(file.lines()
        .flat_map(|line| line.unwrap().parse::<i32>()));

    println!("{:?}", result);
}
