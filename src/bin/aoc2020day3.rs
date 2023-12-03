fn main() {
    let pattern = vec!["..##.......",
                                "#...#...#..",
                                ".#....#..#.",
                                "..#.#...#.#",
                                ".#...##..#.",
                                "..#.##.....",
                                ".#.#.#....#",
                                ".#........#",
                                "#.##...#...",
                                "#...##....#",
                                ".#..#...#.#"];

    let deltax = 1;
    
    let mut x = 0;
    let mut y = 0;
    let mut tree_count = 0;
    let grid_width = pattern[0].len();

    while y < pattern.len() {
        let row_chars:Vec<char> = pattern[y].chars().collect();
        if row_chars[x] == '#' {
            tree_count += 1;
        } 
        y += 1;
        x = (x + deltax) % grid_width;
    }

    println!("trees hit = {}", tree_count);
}
