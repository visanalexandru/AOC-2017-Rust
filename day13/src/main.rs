use std::fs::File;
use std::io::{BufRead, BufReader};

struct Layer {
    depth: u32,
    range: u32,
}

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

fn parse_input(lines: &Vec<String>) -> Vec<Layer> {
    let mut result = Vec::new();
    for line in lines {
        let nums: Vec<&str> = line.split(":").collect();
        let depth = nums[0].trim().parse().unwrap();
        let range = nums[1].trim().parse().unwrap();

        result.push(Layer { depth, range });
    }
    result
}

fn solve1(layers: &Vec<Layer>) -> u32 {
    let mut result = 0;

    for layer in layers {
        let time_to_repeat = 2 * layer.range - 2;
        let mut position = layer.depth % time_to_repeat;

        if position >= layer.range {
            let offset = position - layer.range + 1;
            position = layer.range - offset - 1;
        }

        if position == 0 {
            result += layer.depth * layer.range
        }
    }
    result
}

fn solve2(layers: &Vec<Layer>) -> u32 {
    let mut time = 0;
    loop {
        let mut good = true;
        for layer in layers {
            let time_to_repeat = 2 * layer.range - 2;
            if (time + layer.depth) % time_to_repeat == 0 {
                good = false;
                break;
            }
        }
        if good {
            break;
        }
        time += 1;
    }
    time
}

fn main() {
    let lines = get_lines("input");
    let layers = parse_input(&lines);
    println!("{}", solve1(&layers));
    println!("{}", solve2(&layers));
}
