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

//Returns a list of numbers
fn get_program(lines: &Vec<String>) -> Vec<i32> {
    let mut result = Vec::new();

    for line in lines {
        let data = line.parse().unwrap();
        result.push(data);
    }

    result
}

fn solve1(instructions: &mut Vec<i32>) -> u32 {
    let mut counter: i32 = 0;
    let size: i32 = instructions.len().try_into().unwrap();
    let mut jumps: u32 = 0;

    while counter < size {
        let here: usize = counter.try_into().unwrap();
        let offset = instructions[here];
        counter += offset;
        instructions[here] += 1;
        jumps += 1;
    }

    jumps
}

fn solve2(instructions: &mut Vec<i32>) -> u32 {
    let mut counter: i32 = 0;
    let size: i32 = instructions.len().try_into().unwrap();
    let mut jumps: u32 = 0;

    while counter < size {
        let here: usize = counter.try_into().unwrap();
        let offset = instructions[here];
        counter += offset;

        if instructions[here] >= 3 {
            instructions[here] -= 1;
        } else {
            instructions[here] += 1;
        }

        jumps += 1;
    }

    jumps
}

fn main() {
    let lines = get_lines("input");
    let program = get_program(&lines);

    println!("{}", solve1(&mut program.clone()));
    println!("{}", solve2(&mut program.clone()));
}
