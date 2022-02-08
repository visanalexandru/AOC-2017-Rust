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

fn solve(data: &Vec<char>, index: &mut usize, depth: u32) -> (u32, u32) {
    *index += 1;
    let mut group_sum = depth;
    let mut garbage = 0;

    while data[*index] != '}' {
        match data[*index] {
            '{' => {
                let next = solve(data, index, depth + 1);
                group_sum += next.0;
                garbage += next.1;
                *index += 1;
            }

            '<' => {
                *index += 1;
                while data[*index] != '>' {
                    if data[*index] == '!' {
                        *index += 1;
                    } else {
                        garbage += 1;
                    }
                    *index += 1;
                }
                *index += 1;
            }

            ',' => {
                *index += 1;
            }

            other => {
                panic!("Unknown token: {}", other);
            }
        }
    }

    (group_sum, garbage)
}

fn main() {
    let lines = get_lines("input");
    let input: Vec<char> = lines[0].chars().collect();
    let result = solve(&input, &mut 0, 1);
    println!("{}", result.0);
    println!("{}", result.1);
}
