
fn chain_tuples(tuples: &mut Vec<(i32, i32)>) -> bool {
    let mut head = tuples[0];
    
    let mut index = 1;
    let tuple_count = tuples.len();
    
    let mut is_valid = true;
    let mut swap_count = 1; // 1 less swap than tuples max 
    
    while index < tuple_count {
        
        let other_tuple = tuples[index];
        if head.1 == other_tuple.0 {
            if index > 1 {
                tuples.remove(index);
                tuples.insert(1, other_tuple);
                swap_count +=1;
            } 
        } else if other_tuple.1 == head.0 {
            tuples.remove(index); 
            tuples.insert(0, other_tuple);
            head = other_tuple;
            index = 0;
            is_valid = true;
            swap_count +=1;
        } 
        
        if index + 1 < tuple_count {
            is_valid &= tuples[index].1 == tuples[index + 1].0;
        } 
        
        if swap_count > tuple_count {
            is_valid = false;
            break;
        }
          
        index += 1;
            
    }

    is_valid
}


fn main() {
    let mut input:Vec<(i32, i32)> = vec![(23,42),(8,15),(16,23),(4,8),(15,16)];
    
    println!("{:?}", input);
    
    chain_tuples(&mut input);

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
        test_valid(vec![(23,42),(8,15),(16,23),(4,8),(15,16)],
                    vec![(4,8),(8,15),(15,16),(16,23),(23,42)]);
    }

    #[test]
    fn no_bueno() {
        test_not_valid(vec![(23,42),(8,15),(16,24),(4,8),(15,16)]);
    }
}