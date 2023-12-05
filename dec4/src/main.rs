use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut matches = vec![];
    for line in input.lines().map(|l| l.split(":").nth(1).unwrap()) {
        let winning = line.split("|").nth(0).unwrap().split_whitespace().map(|d| d.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let have = line.split("|").nth(1).unwrap().split_whitespace().map(|d| d.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        println!("win {:?} have {:?}", winning, have);
        let have_winning = have.iter().fold(0, |acc, e| acc + if winning.contains(e) { 1 } else { 0 });
        matches.push(have_winning);
    }

    println!("{:?}", matches);

    let mut counts = vec![1; matches.len()];
    for i in 0..counts.len() {
        for j in 1..=matches[i] {
            counts[i + j] += counts[i];
        }
    }
    println!("{:?}", counts);
    println!("{}", counts.iter().sum::<i32>());
}
