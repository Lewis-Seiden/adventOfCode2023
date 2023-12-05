use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut sum = 0;

    for line in input.lines() {
        let id = line.split(":").next().unwrap().split_ascii_whitespace().last().unwrap().parse::<u64>().unwrap();
        println!("{}", id);
        let (mut red_max, mut green_max, mut blue_max) = (0, 0, 0);
        for round in line.split(":").last().unwrap().split(";") {
            println!("{}", round);
            for (num, color) in round.split(",").map(|s| {
                let mut v = s.split_ascii_whitespace();
                (v.next().unwrap().parse::<u64>().unwrap(), v.next().unwrap())
            }) {
                // println!("{:?} + {:?}", num, color);
                if color == "red" && num > red_max {
                    red_max = num;
                } else if color == "green" && num > green_max {
                    green_max = num;
                } else if color == "blue" && num > blue_max {
                    blue_max = num;
                }
            }
        }
        println!("r{} g{} b{}", red_max, green_max, blue_max);
        sum += red_max * green_max * blue_max;
    }

    println!("{}", sum);
}
