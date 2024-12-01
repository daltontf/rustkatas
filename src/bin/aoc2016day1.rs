use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,   
    West
}

impl Direction {
    fn turn(&self, chr: char) -> Direction {
      match (self, chr) {
        (Direction::North, 'R') => Direction::East,  
        (Direction::East,  'R') => Direction::South, 
        (Direction::South, 'R') => Direction::West,
        (Direction::West,  'R') => Direction::North,
        (Direction::North, 'L') => Direction::West,  
        (Direction::East,  'L') => Direction::North, 
        (Direction::South, 'L') => Direction::East,
        (Direction::West,  'L') => Direction::South,
        _ => *self
      }      
    }
}


struct State {
    direction: Direction,
    x: i32,
    y: i32
}

impl State {
    fn go(&self, chr: char, distance:usize) -> State {
        let new_direction = self.direction.turn(chr);
        let (new_x, new_y) = if new_direction == Direction::North {
            (self.x, self.y + distance as i32)
        } else if new_direction == Direction::East {
            (self.x + distance as i32, self.y)
        } else if new_direction == Direction::South {
            (self.x, self.y - distance as i32)
        } else if new_direction == Direction::West {
            (self.x - distance as i32, self.y)
        } else {
            (self.x, self.y)
        };

        State {
            direction: new_direction,
            x: new_x,
            y: new_y 
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Line {
    Horizontal(i32, i32, i32),
    Vertical(i32, i32, i32),
}

struct State2 {
    direction: Direction,
    x: i32,
    y: i32,
    past_lines: Vec<Line>   
}

impl State2 {
    fn go(&mut self, chr: char, distance:usize) -> &mut State2 {
        let new_direction = self.direction.turn(chr);
        let (new_x, new_y, new_line) = if new_direction == Direction::North {
            (self.x, self.y + distance as i32, Line::Vertical(self.x, self.y, distance as i32))
        } else if new_direction == Direction::East {
            (self.x + distance as i32, self.y, Line::Horizontal(self.x, self.y, distance as i32))
        } else if new_direction == Direction::South {
            (self.x, self.y - distance as i32, Line::Vertical(self.x, self.y, -(distance as i32)))
        } else if new_direction == Direction::West {
            (self.x - distance as i32, self.y, Line::Horizontal(self.x, self.y, -(distance as i32)))
        } else {
            (self.x, self.y, Line::Vertical(self.x, self.y, 0))
        };

        self.x = new_x;
        self.y = new_y;
        self.direction = new_direction;
        self.past_lines.push(new_line);

        self
    }

    fn find_intersection(&self) -> &Line {
        for other in self.past_lines {
            match (&self, line2) {
                (Line::Horizontal(xh, yh, deltah), Line::Vertical(xv, yv, deltav))
              | (Line::Vertical(xv, yv, deltav), Line::Horizontal(xh, yh, deltah)) => {
                    if (xh <= xv)
                        && (xv <= (xh + deltah as i32))
                        && (yv <= yh)
                        && (yh <= (yv + deltav as i32))
                    {
                        Some((xv, yh))
                    } else {
                        None
                    }
                }
                _ => None, // two horizontal or two vertical lines can't "cross" each other in this problem
            }   
        }
    }   
    

    fn new() -> State2 {
        State2 {
            direction: Direction::North, 
            x: 0, y: 0,
            past_lines: Vec::new()
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();

    let part1_state: State = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .flat_map(|line| line.split(',').map(|str| str.to_string()).collect::<Vec<_>>())
        .map(|str: String| {
            let trimmed = str.trim();
            (trimmed.chars().next().unwrap(), trimmed[1..].parse::<usize>().unwrap())            
        })
        .fold(State { direction: Direction::North, x: 0, y: 0 }, |state, char_dist| {
            state.go(char_dist.0, char_dist.1)
        });

        println!("{}", part1_state.x.abs() + part1_state.y.abs());

}
