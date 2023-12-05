use std::fs;

fn main() {
    let dgt_names = vec![
        ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)
    ];
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut sum = 0;
    for line in input.lines() {
        println!("{}", line);
        let mut first = u64::MAX;
        let mut last = u64::MAX;
        for i in 0..line.len() {
            let char = line.chars().nth(i).unwrap();
            if char.is_digit(10) {
                if first == u64::MAX {
                    first = char.to_digit(10).unwrap();
                } else {
                    last = char.to_digit(10).unwrap();
                }
            } else {
                // spelled char
                for (name, dgt) in &dgt_names {
                    if i + name.len() <= line.len() && line[i..i+name.len()] == **name {
                        println!("match {} w {}", &line[i..i+name.len()], name);
                        if first == u64::MAX {
                            first = *dgt;
                        } else {
                            last = *dgt;
                        }
                    }
                }
            }
        }
        if first == u64::MAX {continue;}
        if last == u64::MAX {last = first;}
        println!("({}, {})", first, last);
        sum += (first * 10) + last;
    }
    println!("{}", sum);
}
