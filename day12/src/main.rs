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

//returns the graph given in the input
fn parse_input(lines: &Vec<String>) -> Vec<Vec<usize>> {
    let mut result = Vec::new();

    for line in lines {
        let tokens: Vec<&str> = line.split("<->").collect();

        let origin: usize = tokens[0].trim().parse().unwrap();

        if origin >= result.len() {
            result.resize(origin + 1, Vec::new());
        }

        let neighbours: Vec<&str> = tokens[1].split(",").collect();

        for neighbour in neighbours {
            let neighbour = neighbour.trim().parse().unwrap();
            result[origin].push(neighbour);
        }
    }
    result
}

fn solve(graph: &Vec<Vec<usize>>) -> (usize, usize) {
    let nodes = graph.len();
    let mut groups = vec![0; nodes];
    let mut num_groups = 1;

    for x in 0..nodes {
        if groups[x] == 0 {
            dfs(x, graph, &mut groups, num_groups);
            num_groups += 1;
        }
    }
    //now we need to count how many nodes are in zero's group
    let needed = groups[0];
    let mut result = 0;
    for x in 0..nodes {
        if groups[x] == needed {
            result += 1;
        }
    }
    (result, num_groups - 1)
}

fn dfs(here: usize, graph: &Vec<Vec<usize>>, groups: &mut Vec<usize>, current_group: usize) {
    groups[here] = current_group;
    for &neighbour in &graph[here] {
        if groups[neighbour] == 0 {
            dfs(neighbour, graph, groups, current_group);
        }
    }
}

fn main() {
    let lines = get_lines("input");
    let graph = parse_input(&lines);
    let result = solve(&graph);
    println!("{}", result.0);
    println!("{}", result.1);
}
