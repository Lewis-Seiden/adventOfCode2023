use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    println!("grid {:?}", grid);

    
}
