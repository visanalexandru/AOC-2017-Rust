use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines(filename: &str) -> Vec<String> {
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

fn solve1(lines: &Vec<String>) -> u32 {
    let mut result = 0;

    for line in lines {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let mut valid = true;
        let mut used = HashSet::new(); // use a hash set to memorize the used strings

        for token in tokens {
            if used.contains(token) {
                valid = false;
                break;
            }
            used.insert(token);
        }

        if valid {
            result += 1;
        }
    }

    result
}

fn solve2(lines: &Vec<String>) -> u32 {
    //same as the first part, but we keep sorted strings into the set to find anagrams
    let mut result = 0;

    for line in lines {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let mut valid = true;
        let mut used = HashSet::new();

        for token in tokens {
            let mut characters: Vec<char> = token.chars().collect();
            characters.sort(); //sort the characters in the string and then get back the string
            let sorted = String::from_iter(characters);

            if used.contains(&sorted) {
                valid = false;
                break;
            }
            used.insert(sorted);
        }

        if valid {
            result += 1;
        }
    }

    result
}

fn main() {
    let lines = read_lines("input");
    println!("{}", solve1(&lines));
    println!("{}", solve2(&lines));
}
