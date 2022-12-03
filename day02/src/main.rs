use std::fs;

mod task1;
mod task2;

fn main() {
    const FILE_PATH: &'static str = "./input";
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");
    task1::solve(contents.clone());
    
    task2::solve(contents);
}
