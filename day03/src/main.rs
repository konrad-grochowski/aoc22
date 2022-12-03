use std::fs;

use crate::solutions::{task1, task2};


mod solutions;

fn main() {
    const FILE_PATH: &'static str = "./input";
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    let result = task1::solve(&contents);
    println!("{result}");
    let result = task2::solve(&contents);
    println!("{result}");
}
