use std::fs::File;
use std::io::{BufRead, BufReader};

enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
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

fn get_moves(input: &str) -> Vec<Move> {
    let tokens: Vec<&str> = input.split(",").collect();
    let mut result = Vec::new();
    for token in tokens {
        let op = token.bytes().next().unwrap();
        let remaining = &token[1..];

        if op == b's' {
            result.push(Move::Spin(remaining.parse().unwrap()));
        } else if op == b'x' {
            let mut operands = remaining.split('/');
            let a = operands.next().unwrap().parse().unwrap();
            let b = operands.next().unwrap().parse().unwrap();
            result.push(Move::Exchange(a, b));
        } else if op == b'p' {
            let mut operands = remaining.split('/');
            let a: char = operands.next().unwrap().parse().unwrap();
            let b: char = operands.next().unwrap().parse().unwrap();
            result.push(Move::Partner(a, b));
        }
    }
    result
}

fn spin(array: &mut [char; 16], x: usize) {
    let copy = &array[16 - x..].to_vec();

    for p in (x..16).rev() {
        array[p] = array[p - x];
    }
    for p in 0..x {
        array[p] = copy[p];
    }
}

fn exchange(array: &mut [char; 16], a: usize, b: usize) {
    let old = array[a];
    array[a] = array[b];
    array[b] = old;
}

fn partner(array: &mut [char; 16], a: char, b: char) {
    let (mut a_index, mut b_index) = (17, 17);

    for x in 0..16 {
        if array[x] == a {
            a_index = x;
        } else if array[x] == b {
            b_index = x;
        }
    }
    assert_ne!(a_index, 17);
    assert_ne!(b_index, 17);
    exchange(array, a_index, b_index);
}

fn apply_moves(array: &mut [char; 16], moves: &Vec<Move>) {
    for m in moves {
        match m {
            Move::Spin(x) => spin(array, *x),
            Move::Exchange(a, b) => exchange(array, *a, *b),
            Move::Partner(a, b) => partner(array, *a, *b),
        }
    }
}

fn to_string(array: &[char; 16]) -> String {
    let mut result = String::new();
    for ch in array {
        result.push(*ch);
    }
    result
}

fn solve1(moves: &Vec<Move>) -> String {
    let mut array = ['0'; 16];
    let offset: usize = (b'a').try_into().unwrap();

    for x in 0usize..16 {
        let character: u8 = (x + offset).try_into().unwrap();
        array[x] = character.try_into().unwrap();
    }

    apply_moves(&mut array, moves);
    to_string(&array)
}

const ITERATIONS: usize = 1_000_000_000;

fn solve2(moves: &Vec<Move>) -> String {
    let mut array = ['0'; 16];
    let offset: usize = (b'a').try_into().unwrap();

    for x in 0usize..16 {
        let character: u8 = (x + offset).try_into().unwrap();
        array[x] = character.try_into().unwrap();
    }

    let mut strings: Vec<String> = Vec::new();
    let sorted = "abcdefghijklmnop".to_string();
    strings.push(sorted.clone());

    loop {
        apply_moves(&mut array, moves);
        let now = to_string(&array);
        if now.eq(&sorted) {
            break;
        }
        strings.push(now.clone());
    }

    let index = ITERATIONS % strings.len();
    strings.remove(index)
}

fn main() {
    let lines = get_lines("input");
    let input = &lines[0];
    let moves = get_moves(input);
    println!("{}", solve1(&moves));
    println!("{}", solve2(&moves));
}
