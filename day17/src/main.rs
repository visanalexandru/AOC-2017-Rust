use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_lines(filename: &str) -> Vec<String> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => panic!("Cannot open file: {}", err),
    };
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut result = Vec::new();
    for line in lines {
        let data = match line {
            Ok(data) => data,
            Err(err) => panic!("Cannot read line: {}", err),
        };
        result.push(data);
    }
    result
}

fn solve1(jump: usize) -> u32 {
    const STEPS: usize = 2017;
    let mut values = vec![0];
    let mut current = 0;
    let mut value = 1;

    for _ in 0..STEPS {
        current = (current + jump) % values.len();
        values.insert(current + 1, value);
        current = current + 1;
        value += 1;
    }
    let next = (current + 1) % values.len();
    values[next]
}

fn solve2(jump: usize) -> u32 {
    const STEPS: usize = 50_000_000;

    let mut result = 0;
    let mut len = 1;
    let mut value = 1;
    let mut current = 0;

    for _ in 0..STEPS {
        current = (current + jump) % len;
        if current == 0 {
            result = value;
        }
        len += 1;
        current += 1;
        value += 1;
    }

    result
}

fn main() {
    let lines = get_lines("input");
    let jump: usize = lines[0].parse().unwrap();
    println!("{}", solve1(jump));
    println!("{}", solve2(jump));
}
