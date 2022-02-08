use std::collections::HashMap;
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
        let line = match line {
            Ok(data) => data,
            Err(err) => panic!("Cannot read line: {}", err),
        };
        result.push(line);
    }
    result
}

fn solve(lines: &Vec<String>) -> (i32, i32) {
    let mut registers: HashMap<String, i32> = HashMap::new();
    let mut highest_value = 0;

    for line in lines {
        let tokens: Vec<&str> = line.split_whitespace().collect();

        let (destination, operation, operand, compare) =
            (tokens[0], tokens[1], tokens[4], tokens[5]);

        let dest_value = match registers.get(destination) {
            Some(&x) => x,
            None => 0,
        };

        let operand_value = match registers.get(operand) {
            Some(&x) => x,
            None => 0,
        };

        let op_value: i32 = tokens[2].parse().unwrap();
        let cmp_value: i32 = tokens[6].parse().unwrap();

        let execute = match compare {
            "<" => operand_value < cmp_value,
            "<=" => operand_value <= cmp_value,
            "==" => operand_value == cmp_value,
            "!=" => operand_value != cmp_value,
            ">" => operand_value > cmp_value,
            ">=" => operand_value >= cmp_value,
            other => panic!("Unknown operation: {}", other),
        };

        if execute {
            let new_value = match operation {
                "inc" => dest_value + op_value,
                "dec" => dest_value - op_value,
                other => panic!("Unknown operation: {}", other),
            };

            if new_value > highest_value {
                highest_value = new_value;
            }

            registers.insert(destination.to_string(), new_value);
        }
    }

    let mut maximum = i32::MIN;
    for (_, v) in registers {
        if v > maximum {
            maximum = v;
        }
    }
    (maximum, highest_value)
}

fn main() {
    let lines = get_lines("input");
    let result = solve(&lines);
    println!("{} {}", result.0, result.1);
}
