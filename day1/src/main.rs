use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines(file: &str) -> Vec<String> {
    let f = match File::open(file) {
        Ok(file) => file,
        Err(err) => panic!("Cannot open file: {}", err),
    };
    let reader = BufReader::new(f);
    let lines = reader.lines();
    let mut result = Vec::new();

    for x in lines {
        let data = match x {
            Ok(string) => string,
            Err(err) => panic!("Cannot read line: {}", err),
        };
        result.push(data);
    }

    result
}

fn solve1(digits: &str) -> u32 {
    let chars: Vec<char> = digits.chars().collect();
    let size = chars.len();
    let mut sum = 0;

    for x in 0..size - 1 {
        if chars[x] == chars[x + 1] {
            sum += chars[x].to_digit(10).unwrap();
        }
    }
    if chars[size - 1] == chars[0] {
        sum += chars[size - 1].to_digit(10).unwrap();
    }

    sum
}

fn solve2(digits: &str) -> u32 {
    let chars: Vec<char> = digits.chars().collect();
    let size = chars.len();
    let mut sum = 0;

    for x in 0..size {
        let next = (x + size / 2) % size;

        if chars[x] == chars[next] {
            sum += chars[x].to_digit(10).unwrap();
        }
    }

    sum
}

fn main() {
    let lines = read_lines("input");
    let input = &lines[0];
    println!("{}", solve1(input));
    println!("{}", solve2(input));
}
