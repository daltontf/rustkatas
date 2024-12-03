use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use std::cell::RefCell;
use std::rc::Rc;

use regex::Regex;

enum Operation {
    Bind,
    And,
    Or,
    Not,
    RShift,
    LShift,
    Noop 
}

impl Operation {
    fn apply_arity_1(&self, input: u16) -> Option<u16> {
        match *self {
            Operation::Not => Some(!input),
            Operation::Bind => Some(input),
            _ => Option::None

        }
    }

    fn apply_arity_2(&self, input1: u16, input2: u16) -> Option<u16> {
        match *self {
            Operation::And => Some(input1 & input2),
            Operation::Or => Some(input1 | input2),
            Operation::RShift => Some(input1 >> input2),
            Operation::LShift => Some(input1 << input2),
            _ => Option::None
        }
    }
}

struct Formula {
    operation: Operation,
    input_name_1:String,   
    input_name_2: Option<String>,
    input_constant: Option<u16>,
    output_name: String,
}

impl Formula {
    fn arity_1(operation: Operation, input_name_1:&str, output_name: &str) -> Self {
        Formula {
            operation,
            input_name_1: input_name_1.to_string(),
            input_name_2: Option::None,
            input_constant: Option::None,
            output_name: output_name.to_string(),
        }
    }

    fn arity_1_with_constant(operation: Operation, input_name_1:&str, input_constant: u16, output_name: &str) -> Self {
        Formula {
            operation,
            input_name_1: input_name_1.to_string(),
            input_name_2: Option::None,
            input_constant: Some(input_constant),
            output_name: output_name.to_string(),
        }
    }

    fn arity_2(operation: Operation, input_name_1:&str, input_name_2: &str, output_name: &str) -> Self {
        Formula {
            operation,
            input_name_1: input_name_1.to_string(),
            input_name_2: Some(input_name_2.to_string()),
            input_constant: Option::None,
            output_name: output_name.to_string(),
        }
    }

    fn is_arity_2(&self) -> bool {
        self.input_name_2.is_some()
    }

    fn is_arity_1_with_constant(&self) -> bool {
        self.input_constant.is_some()
    }
} 

struct Engine {
    input_map: HashMap<String, Vec<Rc<RefCell<Formula>>>>,
    value_map: HashMap<String, u16>,
}

impl Engine {
    fn new()-> Engine {
        Engine {
            input_map: HashMap::new(),
            value_map: HashMap::new(),
        }
    }

    fn get_value(&self, key:&str) -> Option<&u16> {
        self.value_map.get(key)
    }

    fn apply_formula(&self, formula: &Formula) -> Option<u16> {       
        if let Some(value1) = self.get_value(formula.input_name_1.as_ref()) {
            if formula.is_arity_2() {
                if let Some(value2) = self.get_value(formula.input_name_2.as_ref().unwrap()) {
                    formula.operation.apply_arity_2(*value1, *value2)
                } else {
                    Option::None
                }
            } else if formula.is_arity_1_with_constant() {
                formula.operation.apply_arity_2(*value1, formula.input_constant.unwrap())
            } else {
                formula.operation.apply_arity_1(*value1)
            }
        } else {
            Option::None
        }       
    }
   
    fn set_value(&mut self, value_name: &str, input_value: u16) {
        let value = self.get_value(value_name);

        if Some(&input_value) != value {
            self.value_map.insert(value_name.to_string(), input_value);

            let mut changes:Vec<(String, u16)> = Vec::new();

            if let Some(formulas) = self.input_map.get(value_name){
                for formula in formulas {
                    let borrowed_formula = formula.borrow_mut();
                    if let Some(output_value) = self.apply_formula(&borrowed_formula) {
                        changes.push((borrowed_formula.output_name.clone(), output_value))
                    }                                               
                }
            }

            for (name, value) in changes {
                self.set_value(&name, value)
            }
        }    
    }

    fn bind_value(&mut self, input_1:&str, output:&str) {
        let rc_formula = Rc::new(RefCell::new(
            Formula::arity_1(Operation::Bind, input_1, output)));
        
        self.add_input(input_1, &rc_formula);

        let borrowed_formula = rc_formula.borrow_mut();
        if let Some(output_value) = self.apply_formula(&borrowed_formula) {
            self.set_value(output, output_value)
        }     
    }

    fn add_input(&mut self, name:&str, rc_formula: &Rc<RefCell<Formula>>) {
        self.input_map.entry(name.to_string())
        .or_default().push(Rc::clone(&rc_formula));   
    }

    fn add_arity1_formula(&mut self, operation:Operation, input_1: &str, output: &str) {
        let rc_formula = Rc::new(RefCell::new(
            Formula::arity_1(operation, input_1, output)));
        
        self.add_input(input_1, &rc_formula); 

        let borrowed_formula = rc_formula.borrow_mut();
        if let Some(output_value) = self.apply_formula(&borrowed_formula) {
            self.set_value(output, output_value)
        }
    }

    fn add_arity1_constant_formula(&mut self, operation:Operation, input_1: &str, value_2: u16, output: &str) {
        let rc_formula = Rc::new(RefCell::new(
            Formula::arity_1_with_constant(operation, input_1, value_2, output)));
        
        self.add_input(input_1, &rc_formula);

        let borrowed_formula = rc_formula.borrow_mut();
        if let Some(output_value) = self.apply_formula(&borrowed_formula) {
            self.set_value(output, output_value)
        }
    }

    fn add_arity2_formula(&mut self, operation:Operation, input_1: &str, input_2: &str, output: &str) {
        let rc_formula = Rc::new(RefCell::new(
            Formula::arity_2(operation, input_1, input_2, output)));
        
        self.add_input(input_1, &rc_formula);
        self.add_input(input_2, &rc_formula);

        let borrowed_formula = rc_formula.borrow_mut();
        if let Some(output_value) = self.apply_formula(&borrowed_formula) {
            self.set_value(output, output_value)
        }
    }

    fn clear_values(&mut self) {
       self.value_map.clear();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let set_constant_value_re = Regex::new(r"^(\d+) -> ([a-z])").unwrap();
    let bind_re = Regex::new(r"^([a-z]+) -> ([a-z]+)").unwrap();
    let not_re = Regex::new(r"^NOT ([a-z]+) -> ([a-z]+)").unwrap();
    let shift_constant_re = Regex::new(r"^([a-z]+) (RSHIFT|LSHIFT) (\d+) -> ([a-z]+)").unwrap();
    let arity_2_re = Regex::new(r"^([a-z]+) (AND|OR) ([a-z]+) -> ([a-z]+)").unwrap();
    let and_or_constant_re = Regex::new(r"^(\d+) (AND|OR) ([a-z]+) -> ([a-z]+)").unwrap();

    let file = File::open(&args[1]).unwrap();

    let mut engine = Engine::new();

    for line in BufReader::new(file)
        .lines()
        .map(|line| line.unwrap()) {
            if let Some(capture) = set_constant_value_re.captures_at(&line, 0) {
                engine.set_value(
                    capture[2].as_ref(),
                    capture[1].parse().unwrap())
            } else if let Some(capture) = bind_re.captures_at(&line, 0) {
                engine.bind_value(capture[1].as_ref(), capture[2].as_ref())
            } else if let Some(capture) = not_re.captures_at(&line, 0) {
                engine.add_arity1_formula(Operation::Not, capture[1].as_ref(), capture[2].as_ref()); 
            } else if let Some(capture) = shift_constant_re.captures_at(&line, 0) {
                let operation = match &capture[2] {
                    "RSHIFT" => Operation::RShift,
                    "LSHIFT" => Operation::LShift,
                    _ => Operation::Noop
                };
                engine.add_arity1_constant_formula(operation, capture[1].as_ref(), capture[3].parse().unwrap(), capture[4].as_ref());
            } else if let Some(capture) = and_or_constant_re.captures_at(&line, 0) {
                let operation = match &capture[2] {
                    "AND" => Operation::And,
                    "OR" => Operation::Or,
                    _ => Operation::Noop
                };
                engine.add_arity1_constant_formula(operation, capture[3].as_ref(), capture[1].parse().unwrap(), capture[4].as_ref());
            } else if let Some(capture) = arity_2_re.captures_at(&line, 0) {
                let operation = match &capture[2] {
                    "AND" => Operation::And,
                    "OR" => Operation::Or,
                    _ => Operation::Noop
                };
                engine.add_arity2_formula(operation, capture[1].as_ref(), capture[3].as_ref(), capture[4].as_ref())
            } else {
                println!("non match: {}", line);
            }
        }

    let a = *engine.value_map.get("a").unwrap();

    println!("a = {:?}", a);

    //engine.set_value("b", a);

    //engine.clear_values();

    engine.delete_output("b");

    
    engine.set_value("b", a);

    println!("a = {:?}", *engine.value_map.get("a").unwrap());
   
    
}