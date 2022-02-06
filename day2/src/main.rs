use std::cmp::{max, min};
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
    for line in lines {
        let data = match line {
            Ok(data) => data,
            Err(err) => panic!("Cannot read line: {}", err),
        };
        result.push(data);
    }
    result
}

//splits the line into numbers
fn get_numbers(line: &str) -> Vec<u32> {
    let split: Vec<&str> = line.split_whitespace().collect();
    let mut result: Vec<u32> = Vec::new();
    for token in split {
        result.push(token.parse().unwrap());
    }
    result
}

fn min_and_max(numbers: &Vec<u32>) -> (u32, u32) {
    let first: u32 = numbers[0];
    let (mut maximum, mut minimum): (u32, u32) = (first, first);

    for number in numbers {
        minimum = min(minimum, *number);
        maximum = max(maximum, *number);
    }

    (minimum, maximum)
}

fn division(numbers: &Vec<u32>) -> u32 {
    let size = numbers.len();
    for x in 0..size {
        for y in x + 1..size {
            if numbers[x] % numbers[y] == 0 {
                return numbers[x] / numbers[y];
            }
            if numbers[y] % numbers[x] == 0 {
                return numbers[y] / numbers[x];
            }
        }
    }

    panic!("Didn't found any good numbers");
}

fn solve1(lines: &Vec<String>) -> u32 {
    let mut result = 0;
    for line in lines {
        let numbers = get_numbers(&line);
        let tup = min_and_max(&numbers);
        result += tup.1 - tup.0;
    }

    result
}

fn solve2(lines: &Vec<String>) -> u32 {
    let mut result = 0;
    for line in lines {
        let numbers = get_numbers(&line);
        result += division(&numbers);
    }
    result
}

fn main() {
    let lines = read_lines("input");
    println!("{}", solve1(&lines));
    println!("{}", solve2(&lines));
}
