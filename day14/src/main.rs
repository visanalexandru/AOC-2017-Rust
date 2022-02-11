use std::fs::File;
use std::io::{BufRead, BufReader};

const COLUMNS: usize = 128;
const LINES: usize = 128;

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

fn hash(mut characters: Vec<u8>) -> String {
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

fn get_binary(hex_string: &String) -> Vec<u8> {
    let mut result = Vec::new();
    for character in hex_string.chars() {
        let binary = match character {
            '0' => [0, 0, 0, 0],
            '1' => [0, 0, 0, 1],
            '2' => [0, 0, 1, 0],
            '3' => [0, 0, 1, 1],
            '4' => [0, 1, 0, 0],
            '5' => [0, 1, 0, 1],
            '6' => [0, 1, 1, 0],
            '7' => [0, 1, 1, 1],
            '8' => [1, 0, 0, 0],
            '9' => [1, 0, 0, 1],
            'a' => [1, 0, 1, 0],
            'b' => [1, 0, 1, 1],
            'c' => [1, 1, 0, 0],
            'd' => [1, 1, 0, 1],
            'e' => [1, 1, 1, 0],
            'f' => [1, 1, 1, 1],
            other => panic!("Unknown hex character: {}", other),
        };
        result.extend_from_slice(&binary);
    }
    result
}

fn get_grid(key: &String) -> Vec<Vec<u8>> {
    let mut result = Vec::new();

    for x in 0..128 {
        let mut row_key = key.clone();
        row_key.push_str("-");
        row_key.push_str(&x.to_string());
        let characters: Vec<u8> = row_key.as_bytes().to_vec();
        let hash = hash(characters);
        result.push(get_binary(&hash));
    }
    result
}

fn solve1(grid: &Vec<Vec<u8>>) -> u32 {
    let mut result = 0;
    for line in grid {
        for &character in line {
            if character == 1 {
                result += 1;
            }
        }
    }
    result
}

fn fill(position: (usize, usize), grid: &Vec<Vec<u8>>, used: &mut Vec<Vec<bool>>) {
    used[position.0][position.1] = true;

    if position.0 > 0 && !used[position.0 - 1][position.1] && grid[position.0 - 1][position.1] == 1
    {
        fill((position.0 - 1, position.1), grid, used);
    }

    if position.0 + 1 < LINES
        && !used[position.0 + 1][position.1]
        && grid[position.0 + 1][position.1] == 1
    {
        fill((position.0 + 1, position.1), grid, used);
    }

    if position.1 > 0 && !used[position.0][position.1 - 1] && grid[position.0][position.1 - 1] == 1
    {
        fill((position.0, position.1 - 1), grid, used);
    }

    if position.1 + 1 < COLUMNS
        && !used[position.0][position.1 + 1]
        && grid[position.0][position.1 + 1] == 1
    {
        fill((position.0, position.1 + 1), grid, used);
    }
}

fn solve2(grid: &Vec<Vec<u8>>) -> u32 {
    let mut used = vec![vec![false; COLUMNS]; LINES];
    let mut result = 0;

    for y in 0..LINES {
        for x in 0..COLUMNS {
            if grid[y][x] == 1 && !used[y][x] {
                fill((y, x), &grid, &mut used);
                result += 1;
            }
        }
    }

    result
}

fn main() {
    let lines = get_lines("input");
    let key = &lines[0];
    let grid = get_grid(key);
    println!("{}", solve1(&grid));
    println!("{}", solve2(&grid));
}
