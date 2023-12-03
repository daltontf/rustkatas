use std::io;

struct OcrAnalysis {}

impl OcrAnalysis {
    fn set_bits(bits: &mut u16, slice: &[u8], line: u16) {
        println!(
            "slice = {}{}{}",
            slice[0] as char, slice[1] as char, slice[2] as char
        );
        if slice[2] as char == '|' {
            *bits |= 1 << (line * 3)
        }
        if slice[1] as char == '_' {
            *bits |= 1 << (line * 3 + 1)
        }
        if slice[0] as char == '|' {
            *bits |= 1 << (line * 3 + 2)
        }
    }

    fn execute(lines: Vec<String>) -> Result<Vec<char>, String> {
        if lines.len() > 2 {
            let shortest = lines
                .iter()
                .fold(65536, |accum, value| std::cmp::min(accum, value.len()));
            if shortest % 3 == 0 {
                let mut bitmap = vec![0u16; shortest / 3];
                for i in 0..(shortest / 3) {
                    for l in 0..lines.len() {
                        OcrAnalysis::set_bits(
                            &mut bitmap[i],
                            &lines[l].as_bytes()[i * 3..(i * 3 + 3)],
                            l as u16,
                        );
                    }
                    println!("{:b}", bitmap[i]);
                }
                let mut result = Vec::new();
                for bits in bitmap {
                    result.push(match bits {
                        0b111_101_010 => '0',
                        0b001_001_000 => '1',
                        0b110_011_010 => '2',
                        0b011_011_010 => '3',
                        0b001_111_000 => '4',
                        0b011_110_010 => '5',
                        0b111_110_010 => '6',
                        0b001_001_010 => '7',
                        0b111_111_010 => '8',
                        0b011_111_010 => '9',
                        _ => '?',
                    });
                }
                Result::Ok(result)
            } else {
                Result::Err("shortest line not multiple of 3".to_string())
            }
        } else {
            Result::Err("not enough lines".to_string())
        }
    }
}

fn main() {
    let lines: Vec<String> = (0..3)
        .map(|_| {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("");
            input
        })
        .collect();

    println!("{:?}", OcrAnalysis::execute(lines));
}
