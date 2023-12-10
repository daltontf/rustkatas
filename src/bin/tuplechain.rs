
fn chain_tuples(tuples: &mut Vec<(i32, i32)>) -> bool {
     
    let tuple_count = tuples.len();
    let mut tail = 0;
    let mut index = 1;
    
    while index < tuple_count {     
        if tuples[index].0 == tuples[tail].1 {
            if index > 1 {
                let index_tuple = tuples.remove(index);
                tuples.insert(tail + 1, index_tuple);
                tail += 1;
                index = tail + 1;
            } 
        } else if tuples[index].1 == tuples[0].0 {
            let index_tuple = tuples.remove(index); 
            tuples.insert(0, index_tuple);
            tail += 1;
            index = tail + 1;
        } else {
            index += 1;
        }
    }
    tail + 1 == index 
}


fn main() {
    let mut input:Vec<(i32, i32)> = vec![(4,8),(23,4),(8,15),(16,23),(4,8),(15,16)];
    
    println!("{:?}", input);
    
    println!("valid= {}", chain_tuples(&mut input));

    println!("{:?}", input);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn test_valid(mut tuples:Vec<(i32, i32)>, expected_tuples:Vec<(i32, i32)>) {
        assert_eq!(true, chain_tuples(&mut tuples));
        assert_eq!(expected_tuples, tuples);
    }

    fn test_not_valid(mut tuples:Vec<(i32, i32)>) {
        assert_eq!(false, chain_tuples(&mut tuples));     
    }

    #[test]
    fn happy_path() {
        test_valid(vec![],
                    vec![]);
        test_valid(vec![(4,4)],
                    vec![(4,4)]);
        test_valid(vec![(15,16),(23,42),(8,15),(16,23),(4,8)],
                    vec![(4,8),(8,15),(15,16),(16,23),(23,42)]);
        test_valid(vec![(23,42),(8,15),(16,23),(4,8),(15,16)],
                    vec![(4,8),(8,15),(15,16),(16,23),(23,42)]);
        test_valid(vec![(4,8),(23,42),(8,15),(16,23),(15,16)],
                    vec![(4,8),(8,15),(15,16),(16,23),(23,42)]);
        test_valid(vec![(4,8),(8,15),(15,16),(16,23),(23,42)],
                    vec![(4,8),(8,15),(15,16),(16,23),(23,42)]);  

         // loops which makes output vary by input order
         test_valid(vec![(23,4),(8,15),(16,23),(4,8),(15,16)],
                    vec![(23, 4), (4, 8), (8, 15), (15, 16), (16, 4)]);          
    }

    #[test]
    fn no_bueno() {
        test_not_valid(vec![(23,42),(8,15)]);
        test_not_valid(vec![(23,42),(8,15),(16,24),(4,8),(15,16)]);       
    }
}