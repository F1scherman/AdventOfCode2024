use std::collections::{HashSet, LinkedList};

fn main() {
    let input = include_str!("input.txt");
    part_two(input);
}

fn part_one(input: &str) {
    let mut grid = vec![];
    input.lines().for_each(|line| {
        grid.push(
            line.chars()
                .map(|y| y.to_digit(10).unwrap())
                .collect::<Vec<u32>>(),
        )
    });

    let mut outer_q = LinkedList::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                outer_q.push_back((i, j));
            }
        }
    }

    let mut count = 0;
    while !outer_q.is_empty() {
        let mut q = LinkedList::new();
        let mut visited = HashSet::new();
        q.push_back(outer_q.pop_front().unwrap());
        while !q.is_empty() {
            let current = q.pop_front().unwrap();
            let curr_value = grid[current.0][current.1];
            if curr_value == 9 && !visited.contains(&current) {
                count += 1;
                visited.insert(current);
                continue;
            }
            let up_row = grid.get(current.0 - 1);
            let down_row = grid.get(current.0 + 1);
            let curr_row = grid.get(current.0);

            match up_row {
                None => {}
                Some(row) => {
                    let mid = row.get(current.1);
                    match mid {
                        None => {}
                        Some(val) => {
                            if *val == curr_value + 1 {
                                q.push_back((current.0 - 1, current.1));
                            }
                        }
                    }
                }
            }
            match down_row {
                None => {}
                Some(row) => {
                    let mid = row.get(current.1);
                    match mid {
                        None => {}
                        Some(val) => {
                            if *val == curr_value + 1 {
                                q.push_back((current.0 + 1, current.1));
                            }
                        }
                    }
                }
            }
            match curr_row {
                None => {}
                Some(row) => {
                    let left = row.get(current.1 - 1);
                    let right = row.get(current.1 + 1);

                    match left {
                        None => {}
                        Some(val) => {
                            if *val == curr_value + 1 {
                                q.push_back((current.0, current.1 - 1));
                            }
                        }
                    }
                    match right {
                        None => {}
                        Some(val) => {
                            if *val == curr_value + 1 {
                                q.push_back((current.0, current.1 + 1));
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", count);
}

fn part_two(input: &str) {
    let mut grid = vec![];
    input.lines().for_each(|line| {
        grid.push(
            line.chars()
                .map(|y| y.to_digit(10).unwrap())
                .collect::<Vec<u32>>(),
        )
    });

    let mut q = LinkedList::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                q.push_back((i, j));
            }
        }
    }

    let mut count = 0;

    while !q.is_empty() {
        let current = q.pop_front().unwrap();
        let curr_value = grid[current.0][current.1];
        if curr_value == 9 {
            count += 1;

            continue;
        }
        let up_row = grid.get(current.0 - 1);
        let down_row = grid.get(current.0 + 1);
        let curr_row = grid.get(current.0);

        match up_row {
            None => {}
            Some(row) => {
                let mid = row.get(current.1);
                match mid {
                    None => {}
                    Some(val) => {
                        if *val == curr_value + 1 {
                            q.push_back((current.0 - 1, current.1));
                        }
                    }
                }
            }
        }
        match down_row {
            None => {}
            Some(row) => {
                let mid = row.get(current.1);
                match mid {
                    None => {}
                    Some(val) => {
                        if *val == curr_value + 1 {
                            q.push_back((current.0 + 1, current.1));
                        }
                    }
                }
            }
        }
        match curr_row {
            None => {}
            Some(row) => {
                let left = row.get(current.1 - 1);
                let right = row.get(current.1 + 1);

                match left {
                    None => {}
                    Some(val) => {
                        if *val == curr_value + 1 {
                            q.push_back((current.0, current.1 - 1));
                        }
                    }
                }
                match right {
                    None => {}
                    Some(val) => {
                        if *val == curr_value + 1 {
                            q.push_back((current.0, current.1 + 1));
                        }
                    }
                }
            }
        }
    }

    println!("{}", count);
}
