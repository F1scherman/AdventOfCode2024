fn main() {
    let input = include_str!("input.txt");
    part_two(input);
}

// NOTE: run this file in release mode and/or with panicking integer underflow turned off. it works as intended even when underflows occur
fn part_one(input: &str) {
    let mut grid = input
        .lines()
        .map(|line| line.to_string().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut guard_coords = (0, 0);
    'outer: for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '^' {
                guard_coords = (i, j);
                break 'outer;
            }
        }
    }

    'travel_loop: loop {
        let mut next_guard_coords;
        'next_coords_validity: loop {
            next_guard_coords = match grid[guard_coords.0][guard_coords.1] {
                '^' => {
                    if guard_coords.0 <= 0 || grid[guard_coords.0 - 1][guard_coords.1] != '#' {
                        (guard_coords.0 - 1, guard_coords.1)
                    } else {
                        grid[guard_coords.0][guard_coords.1] = '>';
                        (guard_coords.0, guard_coords.1 + 1)
                    }
                }
                '>' => {
                    if guard_coords.1 + 1 >= grid[0].len()
                        || grid[guard_coords.0][guard_coords.1 + 1] != '#'
                    {
                        (guard_coords.0, guard_coords.1 + 1)
                    } else {
                        grid[guard_coords.0][guard_coords.1] = 'v';
                        (guard_coords.0 + 1, guard_coords.1)
                    }
                }
                'v' => {
                    if guard_coords.0 + 1 >= grid.len()
                        || grid[guard_coords.0 + 1][guard_coords.1] != '#'
                    {
                        (guard_coords.0 + 1, guard_coords.1)
                    } else {
                        grid[guard_coords.0][guard_coords.1] = '<';
                        (guard_coords.0, guard_coords.1 - 1)
                    }
                }
                '<' => {
                    if guard_coords.1 <= 0 || grid[guard_coords.0][guard_coords.1 - 1] != '#' {
                        (guard_coords.0, guard_coords.1 - 1)
                    } else {
                        grid[guard_coords.0][guard_coords.1] = '^';
                        (guard_coords.0 - 1, guard_coords.1)
                    }
                }
                _ => panic!("Malformed symbol!"),
            };
            if next_guard_coords.0 < 0 || next_guard_coords.1 < 0 {
                break 'next_coords_validity;
            }
            if next_guard_coords.0 >= grid.len() || next_guard_coords.1 >= grid[0].len() {
                break 'next_coords_validity;
            }
            if grid[next_guard_coords.0][next_guard_coords.1] != '#' {
                break 'next_coords_validity;
            }
        }
        let current_symbol = grid[guard_coords.0][guard_coords.1];
        grid[guard_coords.0][guard_coords.1] = 'X';
        if next_guard_coords.0 < 0 || next_guard_coords.1 < 0 {
            break 'travel_loop;
        }
        if next_guard_coords.0 >= grid.len() || next_guard_coords.1 >= grid[0].len() {
            break 'travel_loop;
        }
        grid[next_guard_coords.0][next_guard_coords.1] = current_symbol;
        guard_coords = next_guard_coords;
    }

    let sum = grid
        .iter()
        .map(|row| row.iter().filter(|symbol| **symbol == 'X').count())
        .sum::<usize>();
    println!("{}", sum);
}

fn part_two(input: &str) {
    let original_grid = input
        .lines()
        .map(|line| line.to_string().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut starting_guard_coords = (0, 0);
    'outer: for i in 0..original_grid.len() {
        for j in 0..original_grid[i].len() {
            if original_grid[i][j] == '^' {
                starting_guard_coords = (i, j);
                break 'outer;
            }
        }
    }

    let mut sum = 0;

    for i in 0..original_grid.len() {
        for j in 0..original_grid[i].len() {
            if starting_guard_coords == (i, j) {
                continue;
            }

            let mut grid = original_grid
                .iter()
                .map(|row| row.clone())
                .collect::<Vec<Vec<char>>>();
            grid[i][j] = '#';

            let mut directions_grid = vec![vec![vec![]; grid[0].len()]; grid.len()];
            let mut guard_coords = starting_guard_coords;
            'travel_loop: loop {
                let mut next_guard_coords;
                'next_coords_validity: loop {
                    next_guard_coords = match grid[guard_coords.0][guard_coords.1] {
                        '^' => {
                            if guard_coords.0 <= 0
                                || grid[guard_coords.0 - 1][guard_coords.1] != '#'
                            {
                                (guard_coords.0 - 1, guard_coords.1)
                            } else {
                                grid[guard_coords.0][guard_coords.1] = '>';
                                (guard_coords.0, guard_coords.1 + 1)
                            }
                        }
                        '>' => {
                            if guard_coords.1 + 1 >= grid[0].len()
                                || grid[guard_coords.0][guard_coords.1 + 1] != '#'
                            {
                                (guard_coords.0, guard_coords.1 + 1)
                            } else {
                                grid[guard_coords.0][guard_coords.1] = 'v';
                                (guard_coords.0 + 1, guard_coords.1)
                            }
                        }
                        'v' => {
                            if guard_coords.0 + 1 >= grid.len()
                                || grid[guard_coords.0 + 1][guard_coords.1] != '#'
                            {
                                (guard_coords.0 + 1, guard_coords.1)
                            } else {
                                grid[guard_coords.0][guard_coords.1] = '<';
                                (guard_coords.0, guard_coords.1 - 1)
                            }
                        }
                        '<' => {
                            if guard_coords.1 <= 0
                                || grid[guard_coords.0][guard_coords.1 - 1] != '#'
                            {
                                (guard_coords.0, guard_coords.1 - 1)
                            } else {
                                grid[guard_coords.0][guard_coords.1] = '^';
                                (guard_coords.0 - 1, guard_coords.1)
                            }
                        }
                        _ => panic!("Malformed symbol!"),
                    };
                    if next_guard_coords.0 < 0 || next_guard_coords.1 < 0 {
                        break 'travel_loop;
                    }
                    if next_guard_coords.0 >= grid.len() || next_guard_coords.1 >= grid[0].len() {
                        break 'travel_loop;
                    }
                    if grid[next_guard_coords.0][next_guard_coords.1] != '#' {
                        break 'next_coords_validity;
                    }
                }
                let current_symbol = grid[guard_coords.0][guard_coords.1];
                grid[guard_coords.0][guard_coords.1] = 'X';
                if next_guard_coords.0 < 0 || next_guard_coords.1 < 0 {
                    break 'travel_loop;
                }
                if next_guard_coords.0 >= grid.len() || next_guard_coords.1 >= grid[0].len() {
                    break 'travel_loop;
                }
                grid[next_guard_coords.0][next_guard_coords.1] = current_symbol;
                let mut prev_directions =
                    &mut directions_grid[next_guard_coords.0][next_guard_coords.1];
                if prev_directions.contains(&current_symbol) {
                    sum += 1;
                    break 'travel_loop;
                } else {
                    prev_directions.push(current_symbol);
                }
                guard_coords = next_guard_coords;
            }
        }
    }

    println!("{}", sum);
}
