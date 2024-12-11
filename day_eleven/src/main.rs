use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    part_two(input);
}

fn part_one(input: &str) {
    let mut stones = input
        .split(' ')
        .map(|x| x.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();

    for _ in 0..25 {
        let mut new_stones = vec![];
        for stone in stones.iter() {
            if *stone == 0 {
                new_stones.push(1);
            } else if stone.to_string().len() % 2 == 0 {
                let stone_string = stone.to_string();
                let stone1 = &stone_string[0..stone_string.len() / 2]
                    .parse::<u128>()
                    .unwrap();
                let stone2 = &stone_string[stone_string.len() / 2..]
                    .parse::<u128>()
                    .unwrap();
                new_stones.push(*stone1);
                new_stones.push(*stone2);
            } else {
                new_stones.push(stone * 2024);
            }
        }
        stones = new_stones;
    }

    println!("{}", stones.len());
}

const PART2_ITERATIONS: u32 = 75;

fn part_two(input: &str) {
    let mut memo = HashMap::new();
    let stones = input
        .split(' ')
        .map(|x| find_stones(0, x.parse::<u128>().unwrap(), &mut memo))
        .sum::<u128>();

    println!("{}", stones);
}

fn find_stones(iteration_num: u32, stone: u128, memo: &mut HashMap<(u32, u128), u128>) -> u128 {
    if memo.contains_key(&(iteration_num, stone)) {
        return memo[&(iteration_num, stone)];
    }
    let mut result: u128;
    if iteration_num == PART2_ITERATIONS {
        result = 1;
    } else if stone == 0 {
        result = find_stones(iteration_num + 1, 1, memo);
    } else if stone.to_string().len() % 2 == 0 {
        let stone_string = stone.to_string();
        let stone1 = &stone_string[0..stone_string.len() / 2]
            .parse::<u128>()
            .unwrap();
        let stone2 = &stone_string[stone_string.len() / 2..]
            .parse::<u128>()
            .unwrap();
        result = find_stones(iteration_num + 1, *stone1, memo) + find_stones(iteration_num + 1, *stone2, memo);
    } else {
        result = find_stones(iteration_num + 1, stone * 2024, memo);
    }
    memo.insert((iteration_num, stone), result);
    result
}
