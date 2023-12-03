
fn invalid(board: &[u8;81], i: usize, guess: u8) -> bool {
	let row = i / 9;
	let col = i % 9;
	for other in 0..9 {
		if board[row * 9 + other] == guess
		|| board[other * 9 + col] == guess
		|| board[(((row/3) * 3) + other/3) * 9 + (((col/3) * 3) + other % 3)] == guess {
			return true;
		}
	}
	return false;
}

fn solve(board: &mut [u8;81]) {
    for i in 0..81 {
        if board[i] == 0 {
            for guess in 1..10 {
                if !invalid(board, i, guess) {
                    board[i] = guess;
                    solve(board);
                } 
            }
            board[i] = 0;
            return;
        }
    }
    for y in 0..9 {
        for x in 0..9 {
            print!("{}", board[y * 9 + x]);
        }
        print!("\n");
    }
}

fn main() {
    let mut board : [u8; 81] = [ 
        0,0,0,0,0,0,0,0,0,
        0,0,0,0,0,3,0,8,5,
        0,0,1,0,2,0,0,0,0,
        0,0,0,5,0,7,0,0,0,
        0,0,4,0,0,0,1,0,0,
        0,9,0,0,0,0,0,0,0,
        5,0,0,0,0,0,0,7,3,
        0,0,2,0,1,0,0,0,0,
        0,0,0,0,4,0,0,0,9 
    ];
    
    let start_time = std::time::SystemTime::now(); 
    
    solve(&mut board);
    
    println!("{} seconds elapsed", start_time.elapsed().unwrap().as_secs());
}
