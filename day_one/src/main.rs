use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    part_two(input);
}

fn part_one(input: &str) {
    let mut left = vec![];
    let mut right = vec![];
    input.lines().for_each(|line| {
        let mut it = line.split("   ");
        left.push(it.next().unwrap().parse::<i32>().unwrap());
        right.push(it.next().unwrap().parse::<i32>().unwrap());
    });
    left.sort();
    right.sort();

    let mut sum = 0;
    for i in 0..left.len() {
        sum += (left[i] - right[i]).abs();
    }

    println!("{}", sum);
}

fn part_two(input: &str) {
    let mut left = vec![];
    let mut right = HashMap::new();
    input.lines().for_each(|line| {
        let mut it = line.split("   ");
        left.push(it.next().unwrap().parse::<i32>().unwrap());
        let right_key = it.next().unwrap().parse::<i32>().unwrap();
        if right.contains_key(&right_key) {
            right.insert(right_key, right[&right_key] + 1);
        } else {
            right.insert(right_key, 1);
        }
    });

    let mut sum = 0;
    for i in left.iter() {
        sum += right.get(i).unwrap_or(&0) * i;
    }

    println!("{}", sum);
}
