use regex::Regex;
use std::collections::{HashMap, HashSet, LinkedList};
use std::str;

fn main() {
    let input = include_str!("input.txt");
    part_two(input);
}

fn part_one(input: &str) {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    let mut sum = 0;

    let rule_regex = Regex::new(r"^(\d+)\|(\d+)$").unwrap();

    input.lines().for_each(|line| {
        if rule_regex.is_match(line) {
            let caps = rule_regex.captures(line).unwrap();
            let vertex_one = caps.get(1).unwrap().as_str();
            let vertex_two = caps.get(2).unwrap().as_str();
            if graph.contains_key(vertex_one) {
                graph
                    .get_mut(vertex_one)
                    .unwrap()
                    .push(vertex_two.to_string());
            } else {
                graph.insert(vertex_one.to_string(), vec![vertex_two.to_string()]);
            }
            return;
        } else if line == "" {
            return;
        }

        let list = parse_list(line);

        if !is_valid_toposort(&graph, &list) {
            return;
        }
        sum += list[list.len() / 2].parse::<i32>().unwrap();
    });

    println!("{}", sum);
}

fn part_two(input: &str) {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    let mut sum = 0;

    let rule_regex = Regex::new(r"^(\d+)\|(\d+)$").unwrap();

    input.lines().for_each(|line| {
        if rule_regex.is_match(line) {
            let caps = rule_regex.captures(line).unwrap();
            let vertex_one = caps.get(1).unwrap().as_str();
            let vertex_two = caps.get(2).unwrap().as_str();
            if graph.contains_key(vertex_one) {
                graph
                    .get_mut(vertex_one)
                    .unwrap()
                    .push(vertex_two.to_string());
            } else {
                graph.insert(vertex_one.to_string(), vec![vertex_two.to_string()]);
            }
            return;
        } else if line == "" {
            return;
        }

        let list = parse_list(line);

        if is_valid_toposort(&graph, &list) {
            return;
        }

        let mut trim_graph = HashMap::new();

        for (src, dst) in graph.iter() {
            if !list.contains(&src) {
                continue;
            }
            let mut neighbors = vec![];
            for cand in dst {
                if list.contains(cand) {
                    neighbors.push(cand.clone());
                }
            }
            trim_graph.insert(src.to_string(), neighbors);
        }

        let mut queue = LinkedList::new();
        let mut toposort = vec![];

        let mut indegrees = HashMap::new();
        for src in trim_graph.keys() {
            indegrees.insert(src.to_string(), 0);
        }
        for neighbors in trim_graph.values() {
            for dst in neighbors {
                if !indegrees.contains_key(dst) {
                    indegrees.insert(dst.to_string(), 0);
                }
                indegrees.insert(
                    dst.to_string(),
                    indegrees.get(&dst.to_string()).unwrap() + 1,
                );
            }
        }

        for (src, count) in indegrees.iter() {
            if *count == 0 {
                queue.push_back(src.to_string());
            }
        }

        while !queue.is_empty() {
            let curr = queue.pop_front().unwrap();
            toposort.push(curr.to_string());

            let neighbors = trim_graph.get(&curr.to_string());
            if neighbors.is_none() {
                continue;
            }
            for neighbor in neighbors.unwrap().iter() {
                let indegrees_new_value = indegrees.get(neighbor).unwrap() - 1;
                indegrees.insert(neighbor.to_string(), indegrees_new_value);
                if indegrees_new_value == 0 {
                    queue.push_back(neighbor.to_string());
                }
            }
        }

        sum += toposort[toposort.len() / 2].parse::<i32>().unwrap();
    });

    println!("{}", sum);
}

fn is_valid_toposort(graph: &HashMap<String, Vec<String>>, list: &Vec<String>) -> bool {
    let mut visited = HashSet::new();
    for x in list.iter() {
        visited.insert((*x).to_string());
    }

    for i in list.iter().rev() {
        let neighbors = graph.get(i);
        if !neighbors.is_none() {
            for j in neighbors.unwrap().iter() {
                if visited.contains(j) {
                    return false;
                }
            }
        }
        visited.remove(i);
    }

    true
}

fn parse_list(line: &str) -> Vec<String> {
    let mut list = vec![];
    let mut current = String::new();
    for i in line.chars() {
        if i == ',' {
            list.push(current.clone());
            current.clear();
        } else {
            current.push(i);
        }
    }
    list.push(current.clone());

    list
}
