use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Direction {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

#[derive(Debug)]
struct HexPosition {
    s: i32,
    r: i32,
    q: i32,
}

impl HexPosition {
    fn advance(&mut self, dir: &Direction) {
        match dir {
            Direction::North => {
                self.r -= 1;
                self.s += 1
            }
            Direction::NorthEast => {
                self.r -= 1;
                self.q += 1
            }
            Direction::SouthEast => {
                self.s -= 1;
                self.q += 1
            }
            Direction::South => {
                self.s -= 1;
                self.r += 1
            }
            Direction::SouthWest => {
                self.q -= 1;
                self.r += 1
            }
            Direction::NorthWest => {
                self.q -= 1;
                self.s += 1
            }
        }
    }
    fn distance_to_origin(&self) -> i32 {
        (self.s.abs() + self.r.abs() + self.q.abs()) / 2
    }
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

fn get_instructions(data: &String) -> Vec<Direction> {
    let tokens = data.split(',');
    let mut result = Vec::new();
    for token in tokens {
        let to_add = match token {
            "n" => Direction::North,
            "ne" => Direction::NorthEast,
            "se" => Direction::SouthEast,
            "s" => Direction::South,
            "sw" => Direction::SouthWest,
            "nw" => Direction::NorthWest,
            x => panic!("Unknown direction: {}", x),
        };
        result.push(to_add);
    }
    result
}

fn solve(instructions: &Vec<Direction>) -> (i32, i32) {
    let mut here = HexPosition { s: 0, r: 0, q: 0 };
    let mut maximum = 0;
    for instruction in instructions {
        here.advance(instruction);
        let distance = here.distance_to_origin();
        if distance > maximum {
            maximum = distance;
        }
    }
    (here.distance_to_origin(), maximum)
}

fn main() {
    let lines = get_lines("input");
    let instructions = get_instructions(&lines[0]);
    let result = solve(&instructions);
    println!("{}", result.0);
    println!("{}", result.1);
}
