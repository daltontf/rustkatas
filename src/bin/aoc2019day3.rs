#[derive(Copy, Clone, Debug)]
enum Line {
    Horizontal(i32, i32, u32),
    Vertical(i32, i32, u32),
}

fn intersection(line1: Line, line2: Line) -> Option<(i32, i32)> {
    match (line1, line2) {
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

fn make_lines_for_path(path: Vec<&str>) -> Vec<Line> {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut result = Vec::new();

    for elem in path {
        let mut chars = elem.chars();
        match (chars.next(), chars.as_str().parse()) {
            (Some('U'), Ok(delta)) => {
                result.push(Line::Vertical(x, y - delta as i32, delta));
                y -= delta as i32;
            }
            (Some('D'), Ok(delta)) => {
                result.push(Line::Vertical(x, y, delta));
                y += delta as i32;
            }
            (Some('L'), Ok(delta)) => {
                result.push(Line::Horizontal(x - delta as i32, y, delta));
                x -= delta as i32;
            }
            (Some('R'), Ok(delta)) => {
                result.push(Line::Horizontal(x, y, delta));
                x += delta as i32;
            }
            _ => (),
        }
    }
    result
}

fn main() {
    let mut answer: Option<(i32, i32)> = None;

    let argv: Vec<String> = std::env::args().collect();

    let lines1 = make_lines_for_path(argv[1].split(",").collect());
    let lines2 = make_lines_for_path(argv[2].split(",").collect());

    for line1 in &lines1 {
        for line2 in &lines2 {
            if let Some((x, y)) = intersection(*line1, *line2) {
                if x != 0 || y != 0 {
                    // Not interested in origin
                    if let Some((answer_x, answer_y)) = answer {
                        if x.abs() + y.abs() < answer_x.abs() + answer_y.abs() {
                            answer = Some((x, y));
                        }
                    } else {
                        answer = Some((x, y));
                    }
                }
            }
        }
    }

    if let Some((x, y)) = answer {
        println!(
            "Nearest = ({}, {}) - distance = {}",
            x,
            y,
            x.abs() + y.abs()
        )
    } else {
        println!("No intersections");
    }
}
