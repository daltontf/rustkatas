
struct State {
    current: Vec<char>
}

impl State {
    fn new(str: &str) -> State {
        State {
            current: str.chars().collect()
        }
    }

    fn increment(&mut self) {
        let mut index = self.current.len() - 1;
        let mut value = self.current[index] as u8 - 97;
        while value == 25 {
            self.current[index] = 'a';
            if index == 0 {
                break;
            }
            index -= 1;
            value = self.current[index] as u8 - 97;
        }
        self.current[index] = char::from_u32(value as u32 + 98).unwrap();
    }

    fn valid(&self) -> bool {
        let mut first_double: Option<char> = Option::None;
        let mut second_double: Option<char> = Option::None;
        let mut last_chr: Option<char> = Option::None;
        let mut ascending: Vec<char> = vec![];
        

        for chr in self.current.iter() {
            if *chr == 'i' || *chr =='l' ||  *chr == 'o' {
                return false;
            }

            if let Some(last) = last_chr {
                if last == *chr {
                    if first_double.is_none() {
                        first_double = Some(*chr);
                    } else if second_double.is_none() {
                        if first_double.unwrap() != *chr {
                            second_double = Some(*chr);
                        }
                    }
                }
                if ascending.len() < 3 {
                    if last as u8 + 1 == *chr as u8 {
                        ascending.push(*chr);
                    } else {
                        ascending.clear();
                        ascending.push(*chr);                   
                    }
                }
            }

            last_chr = Some(*chr);
        }

        ascending.len() > 2 && first_double.is_some() && second_double.is_some()
    }

    fn next(&mut self) {
        loop {
            self.increment();
            if self.valid() {
                break;
            }
        }    
    }
}

fn main() {
    let mut state = State::new("hepxxyzz");

    state.next();

    println!("{:?}", state.current.iter().collect::<String>());

    state.next();

    println!("{:?}", state.current.iter().collect::<String>())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn increment_of(input: &str) -> String {
        let mut state = State::new(input);
        state.increment();
        state.current.iter().collect()
    }

    #[test]
    fn test_increment() {
        assert_eq!("aaab".to_string(), increment_of("aaaa"));
        assert_eq!("aabb".to_string(), increment_of("aaba"));
        assert_eq!("aazj".to_string(), increment_of("aazi"));
        assert_eq!("azaa".to_string(), increment_of("ayzz"));
    }

    #[test]
    fn test_valid() {
        assert_eq!(false, State::new("caabch").valid());
        assert_eq!(true, State::new("caabchh").valid());
        assert_eq!(false, State::new("caabdhh").valid());
        assert_eq!(false, State::new("caabchhi").valid());
    }
}