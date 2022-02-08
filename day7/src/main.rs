use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Program {
    name: String,
    weight: u32,
    children: Vec<String>,
}

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
            Ok(data) => data,
            Err(err) => panic!("Cannot read line: {}", err),
        };
        result.push(data);
    }
    result
}

fn parse_lines(lines: &Vec<String>) -> HashMap<String, Program> {
    let mut result = HashMap::new();

    for line in lines {
        let x = line.replace(",", " ");

        let tokens: Vec<&str> = x.split_whitespace().collect();
        let name = tokens[0].to_string();
        let weight = &tokens[1][1..tokens[1].len() - 1];
        let weight: u32 = weight.parse().unwrap();
        let mut children = Vec::new();

        if tokens.len() > 3 {
            for token in &tokens[3..] {
                children.push(token.to_string());
            }
        }

        let program = Program {
            name,
            weight,
            children,
        };

        result.insert(program.name.clone(), program);
    }

    result
}

fn solve1(map: &HashMap<String, Program>) -> &Program {
    let mut leaf_nodes = HashSet::new();

    for (_, v) in map.iter() {
        for child in &v.children {
            leaf_nodes.insert(child);
        }
    }

    for (k, v) in map.iter() {
        if !leaf_nodes.contains(k) {
            return v;
        }
    }

    panic!("Cannot find the root node!");
}

fn solve2(root: &Program, map: &HashMap<String, Program>) -> u32 {
    let mut weights = HashMap::new();
    let mut invalid = HashMap::new();

    dfs1(root, map, &mut weights, &mut invalid);

    let mut now = root;
    let mut found = true;

    while found {
        found = false;

        for child in &now.children {
            if *invalid.get(child).unwrap() {
                now = map.get(child).unwrap();
                found = true;
            }
        }
    }

    let mut majority = weights.get(&now.children[0]).unwrap();
    let mut count = 0;

    for child in &now.children {
        let here = weights.get(child).unwrap();
        if here == majority {
            count += 1;
        }
    }

    if count == 1 {
        majority = weights.get(&now.children[1]).unwrap();
    }

    let mut change = &now.children[0];
    for child in &now.children {
        if weights.get(child).unwrap() != majority {
            change = &child;
        }
    }
    let weight_to_change = weights.get(change).unwrap();
    let current = map.get(change).unwrap().weight;

    current + majority - weight_to_change
}

//creates the weights table
fn dfs1(
    current: &Program,
    map: &HashMap<String, Program>,
    weights: &mut HashMap<String, u32>,
    invalid: &mut HashMap<String, bool>,
) {
    let mut tower_weight = current.weight;
    let mut child_weights = Vec::new();
    let mut incorrect = false;

    for child in &current.children {
        let next_program = map.get(child).unwrap();
        dfs1(next_program, map, weights, invalid);
        let child_weight = *weights.get(child).unwrap();
        tower_weight += child_weight;

        let child_correct = *invalid.get(child).unwrap();
        incorrect |= child_correct;

        child_weights.push(child_weight);
    }

    for &w in &child_weights {
        if w != child_weights[0] {
            incorrect = true;
            break;
        }
    }

    weights.insert(current.name.clone(), tower_weight);
    invalid.insert(current.name.clone(), incorrect);
}

fn main() {
    let lines = get_lines("input");
    let map = parse_lines(&lines);
    let root = solve1(&map);
    println!("{}", root.name);
    println!("{}", solve2(&root, &map));
}
