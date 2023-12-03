use num_traits::cast::*;
use num_traits::*;

fn average(numbers: &Vec<f64>) -> f64 {
    let sum: f64 = numbers.iter().fold(0 as f64, |acc, n| acc + n);
    sum / numbers.len() as f64
}

fn average2<T: Num + NumCast + Copy + Default>(numbers: &Vec<T>) -> T {
    let sum: T = numbers.iter().fold(Default::default(), |acc, n| acc + *n);
    sum / cast::<usize, T>(numbers.len()).unwrap()
}

fn main() {
    let numbers: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
    let numbersi: Vec<i32> = vec![1, 2, 3, 4];
    println!("average = {}", average(&numbers));
    println!("average2 = {}", average2(&numbersi));
}
