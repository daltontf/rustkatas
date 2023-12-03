fn main() {
    let entries = vec![1721, 979, 366, 299, 675, 1456, 1010, 1010];

    for (i, ientry) in entries.iter().enumerate() {
        for (j, jentry) in entries.iter().enumerate() {
            if i < j && ientry + jentry == 2020 {
                println!("{} + {}", ientry, jentry)
            }
        }
    }
}
