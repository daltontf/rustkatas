fn prepend(item: u16, slice:Vec<u16>) -> Vec<u16> {
    let mut v = vec![item];
    for it in slice.iter() {
        v.push(*it);
    }
    v
}

fn reduce<'a>(ys:&[u16], f: &dyn Fn(&[u16]) -> (u16, &[u16])) -> Vec<u16> {
    match f(ys) {
        (_, []) => Vec::new(),
        (x, list) => prepend(x, reduce(list, f))
    }
}

fn score_frame(balls: &[u16]) -> (u16, &[u16]) {
      match balls {
        [x1, x2, x3,..] if *x1 == 10 => ((x1 + x2 + x3), &balls[1..]),
    	[x1, x2, x3,..] if *x1 + *x2 == 10 => ((x1 + x2 + x3), &balls[2..]),
    	[x1, x2,..] => (x1 + x2, &balls[2..]),
		_ => (0, &[])
     }
}

fn tally_scores(scores: Vec<u16>) -> Vec<u16>  {
    scores.into_iter().fold(
        Vec::new(),
        |mut accum, score| {
            accum.push(accum.last().unwrap_or(&0u16) + score);
            accum
        }
    )
}

fn score_game(balls:&[u16]) -> Vec<u16> {
    reduce(&balls, &score_frame)
}

fn main()  {
    let argv : Result<Vec<u16>,_> = std::env::args().skip(1).map(|s| s.parse()).collect();
    for score in tally_scores(score_game(argv.unwrap().as_slice())) {
        print!("{} ",score);
    }
    println!();
}