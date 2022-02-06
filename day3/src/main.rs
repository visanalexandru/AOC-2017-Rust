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
            Ok(string) => string,
            Err(err) => panic!("Cannot read line: {}", err),
        };
        result.push(data);
    }
    result
}

fn solve1(num: u32) -> u32 {
    let (mut x, mut y): (i32, i32) = (0, 0);
    let mut current = 1;
    let mut size = 1;

    if current == num {
        return 0;
    }
    x += 1;
    current += 1;

    while current < num {
        for _ in 0..size {
            if current == num {
                break;
            }
            y += 1;
            current += 1;
        }

        for _ in 0..size + 1 {
            if current == num {
                break;
            }
            x -= 1;
            current += 1;
        }

        for _ in 0..size + 1 {
            if current == num {
                break;
            }
            y -= 1;
            current += 1;
        }

        for _ in 0..size + 2 {
            if current == num {
                break;
            }
            x += 1;
            current += 1;
        }
        size += 2;
    }

    (x.abs() + y.abs()).try_into().unwrap()
}

fn sum_neighbours(pos: (usize, usize), matrix: &Vec<Vec<u32>>) -> u32 {
    let y: i32 = pos.0.try_into().unwrap();
    let x: i32 = pos.1.try_into().unwrap();
    let mut sum = 0;

    for offy in -1..2 {
        for offx in -1..2 {
            if offy == 0 && offx == 0 {
                continue;
            }

            let newx: usize = (x + offx).try_into().unwrap();
            let newy: usize = (y + offy).try_into().unwrap();

            sum += matrix[newy][newx]
        }
    }

    sum
}

fn solve2(num: u32) -> u32 {
    const WIDTH: usize = 300;
    const HEIGHT: usize = 300;

    let mut matrix = vec![vec![0; WIDTH]; HEIGHT];
    let mut pos: (usize, usize) = (150, 150);
    let mut size = 1;

    matrix[pos.0][pos.1] = 1;
    pos.1 += 1;
    matrix[pos.0][pos.1] = 1;

    while matrix[pos.0][pos.1] <= num {
        for _ in 0..size {
            if matrix[pos.0][pos.1] > num {
                break;
            }

            pos.0 += 1;
            matrix[pos.0][pos.1] = sum_neighbours(pos, &matrix);
        }

        for _ in 0..size + 1 {
            if matrix[pos.0][pos.1] > num {
                break;
            }

            pos.1 -= 1;
            matrix[pos.0][pos.1] = sum_neighbours(pos, &matrix);
        }

        for _ in 0..size + 1 {
            if matrix[pos.0][pos.1] > num {
                break;
            }
            pos.0 -= 1;
            matrix[pos.0][pos.1] = sum_neighbours(pos, &matrix);
        }

        for _ in 0..size + 2 {
            if matrix[pos.0][pos.1] > num {
                break;
            }
            pos.1 += 1;
            matrix[pos.0][pos.1] = sum_neighbours(pos, &matrix);
        }
        size += 2;
    }

    matrix[pos.0][pos.1]
}

fn main() {
    let lines = get_lines("input");
    let number: u32 = lines[0].parse().unwrap();

    println!("{}", solve1(number));
    println!("{}", solve2(number));
}
