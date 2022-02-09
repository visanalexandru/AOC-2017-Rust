use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_lines(filename: &str) -> Vec<String> {
    let file = match File::open(filename) {
        Ok(f) => f,
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

fn get_numbers(nums: &String) -> Vec<usize> {
    let tokens = nums.split(',');
    let mut result = Vec::new();
    for token in tokens {
        result.push(token.parse().unwrap());
    }
    result
}

fn run_round(
    lengths: &Vec<usize>,
    numbers: &mut Vec<usize>,
    current: &mut usize,
    skip_size: &mut usize,
) {
    for &length in lengths {
        let mut copy = Vec::new();
        let mut here = *current;

        for _ in 0..length {
            copy.push(numbers[here]);
            here = (here + 1) % numbers.len();
        }

        here = *current;
        copy.reverse();

        for x in 0..copy.len() {
            numbers[here] = copy[x];
            here = (here + 1) % numbers.len();
        }

        *current = (*current + length + *skip_size) % numbers.len();
        *skip_size += 1;
    }
}

fn solve1(nums: &Vec<usize>) -> usize {
    const SIZE: usize = 256;
    let mut numbers = vec![0; SIZE];

    for x in 0..SIZE {
        numbers[x] = x;
    }

    run_round(nums, &mut numbers, &mut 0, &mut 0);
    return numbers[0] * numbers[1];
}

fn solve2(mut characters: Vec<u8>) -> String {
    const SIZE: usize = 256;
    let mut numbers = vec![0; SIZE];

    for x in 0..SIZE {
        numbers[x] = x;
    }

    characters.extend_from_slice(&[17, 31, 73, 47, 23]); // add the sequence
    let mut lengths: Vec<usize> = Vec::new();
    for x in characters {
        lengths.push(x.try_into().unwrap());
    }

    let (mut current, mut skip_size) = (0, 0);
    for _ in 0..64 {
        run_round(&lengths, &mut numbers, &mut current, &mut skip_size);
    }

    let mut dense_hash = Vec::new();
    let mut now = 0;
    for x in 0..SIZE {
        now ^= numbers[x];

        if x % 16 == 15 {
            dense_hash.push(now);
            now = 0;
        }
    }

    let mut result = String::new();
    for num in dense_hash {
        let hex = format!("{:02x}", num);
        result.push_str(&hex);
    }
    result
}

fn main() {
    let lines = get_lines("input");
    let nums = get_numbers(&lines[0]);
    let characters = lines[0].trim().as_bytes().to_vec();
    println!("{}", solve1(&nums));
    println!("{}", solve2(characters));
}
