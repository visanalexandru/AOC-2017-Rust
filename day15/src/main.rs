use std::fs::File;
use std::io::{BufRead, BufReader};

const MOD: u64 = 2_147_483_647;
const A_FACTOR: u64 = 16_807;
const B_FACTOR: u64 = 48_271;
const MASK: u64 = 0xffff;

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

fn parse_lines(lines: &Vec<String>) -> (u64, u64) {
    let tokens: Vec<&str> = lines[0].split_whitespace().collect();
    let a = tokens[4].parse().unwrap();
    let tokens: Vec<&str> = lines[1].split_whitespace().collect();
    let b = tokens[4].parse().unwrap();

    (a, b)
}

fn solve1(a_seed: u64, b_seed: u64, iterations: u64) -> u32 {
    let (mut a, mut b) = (a_seed, b_seed);
    let mut result = 0;
    for _ in 0..iterations {
        let a_next = (a * A_FACTOR) % MOD;
        let b_next = (b * B_FACTOR) % MOD;
        a = a_next;
        b = b_next;
        if (a & MASK) == (b & MASK) {
            result += 1;
        }
    }
    result
}

fn solve2(a_seed: u64, b_seed: u64, iterations: u64) -> u32 {
    let (mut a, mut b) = (a_seed, b_seed);
    let mut result = 0;
    for _ in 0..iterations {
        let mut a_next = (a * A_FACTOR) % MOD;
        let mut b_next = (b * B_FACTOR) % MOD;

        while (a_next % 4) != 0 {
            a_next = (a_next * A_FACTOR) % MOD;
        }

        while (b_next % 8) != 0 {
            b_next = (b_next * B_FACTOR) % MOD;
        }

        a = a_next;
        b = b_next;

        if (a & MASK) == (b & MASK) {
            result += 1;
        }
    }
    result
}

fn main() {
    let lines = get_lines("input");
    let (a, b) = parse_lines(&lines);
    println!("{}", solve1(a, b, 40_000_000));
    println!("{}", solve2(a, b, 5_000_000));
}
