use std::io;

use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Rect {
    pub x: usize,
    pub y: usize,
    w: usize,
    h: usize,
    pub rendered: bool,
    pub valid: bool,
}

impl Rect {
    fn new(x: usize, y: usize, w: usize, h: usize) -> Rect {
        Rect {
            x,
            y,
            w,
            h,
            rendered: false,
            valid: true,
        }
    }

    fn set_w_from(&mut self, x: usize) -> () {
        self.w = x - self.x;
    }

    fn set_h_from(&mut self, y: usize) -> () {
        self.h = y - self.y;
    }
}

struct RectFinder {
    possible_rects: Vec<Rect>,
    y: usize,
}

impl RectFinder {
    fn new() -> RectFinder {
        RectFinder {
            possible_rects: vec![],
            y: 0,
        }
    }

    fn process_line(&mut self, input: &str) -> () {
        let len = input.len();
        let chrs = input.as_bytes();
        for x in 0..len {
            let mut new_rects: Vec<Rect> = vec![];
            match chrs[x] as char {
                '+' => {
                    for rect in &mut self.possible_rects {
                        if rect.y == self.y && rect.w == 0 {
                            rect.set_w_from(x);
                            new_rects.push(Rect::new(rect.x, rect.y, 0, 0));
                        }
                        if rect.x == x && rect.h == 0 {
                            rect.set_h_from(self.y);
                            new_rects.push(Rect::new(rect.x, rect.y, rect.w, 0));
                        }
                        if rect.x + rect.w == x
                            && rect.y + rect.h == self.y
                            && rect.h > 0
                            && rect.w > 0
                        {
                            rect.rendered = true;
                        }
                    }
                    self.possible_rects.push(Rect::new(x, self.y, 0, 0));
                    self.possible_rects.append(&mut new_rects);
                }
                '|' => {}
                '-' => {}
                _ => { /* invalid entire input? */ }
            }
            for rect in &mut self.possible_rects {
                if rect.y == self.y || rect.y + rect.h == self.y {
                    if len >= rect.x + rect.w {
                        for i in rect.x..rect.x + rect.w {
                            if chrs[i] as char != '-' && chrs[i] as char != '+' {
                                rect.valid = false;
                            }
                        }
                    } else {
                        rect.valid = false;
                    }
                } else if rect.valid && !rect.rendered && rect.y < self.y {
                    // validate that x
                    if (len <= rect.x || chrs[rect.x] as char != '|' && chrs[rect.x] as char != '+')
                        || (len <= rect.x + rect.w
                            || chrs[rect.x + rect.w] as char != '|'
                                && chrs[rect.x + rect.w] as char != '+')
                    {
                        rect.valid = false;
                    }
                }
            }
        }
        self.y += 1;
    }

    fn found_rects(&self) -> Vec<Rect> {
        self.possible_rects
            .clone()
            .into_iter()
            .filter(|rect| rect.rendered && rect.valid)
            .collect()
    }
}

fn main() {
    let mut input = String::new();
    let mut finder = RectFinder::new();

    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1]).unwrap();

    while BufReader::new(file).read_line(&mut input).is_ok() && input.len() > 0 {
        print!("{}", input);
        finder.process_line(input.as_str());
        input = String::new();
    }

    println!();
    for rect in finder.found_rects() {
        println!("{:?}", rect);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn no_input() {
        let finder = RectFinder::new();
        let output = finder.possible_rects;
        assert_eq!(output.len(), 0);
    }

    #[test]
    fn simplest_rect() {
        let mut finder = RectFinder::new();
        finder.process_line("++");
        finder.process_line("++");

        let output = finder.found_rects();
        assert_eq!(output.len(), 1);
        assert_eq!(
            output[0],
            Rect {
                x: 0,
                y: 0,
                w: 1,
                h: 1,
                rendered: true,
                valid: true
            }
        );

        finder.process_line("++");

        let output = finder.found_rects();
        assert_eq!(output.len(), 3);
        assert_eq!(
            output[0],
            Rect {
                x: 0,
                y: 0,
                w: 1,
                h: 1,
                rendered: true,
                valid: true
            }
        );
        assert_eq!(
            output[1],
            Rect {
                x: 0,
                y: 1,
                w: 1,
                h: 1,
                rendered: true,
                valid: true
            }
        );
        assert_eq!(
            output[2],
            Rect {
                x: 0,
                y: 0,
                w: 1,
                h: 2,
                rendered: true,
                valid: true
            }
        );
    }
}
