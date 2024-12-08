use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    part_two(input);
}

fn part_one(input: &str) {
    let mut frequencies = HashSet::new();
    input.lines().for_each(|line| {
        line.chars().for_each(|c| {
            frequencies.insert(c);
        });
    });
    frequencies.remove(&'.');

    let mut locations = HashSet::new();

    for frequency in frequencies.iter() {
        let grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|x| if x != *frequency { '.' } else { *frequency })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut antennas = HashSet::new();
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == *frequency {
                    antennas.insert((i, j));
                }
            }
        }

        for antenna1 in antennas.iter() {
            for antenna2 in antennas.iter() {
                if *antenna1 == *antenna2 {
                    continue;
                }

                let x_diff = antenna1.0.abs_diff(antenna2.0);
                let y_diff = antenna1.1.abs_diff(antenna2.1);
                let negative_x = antenna1.0 > antenna2.0;
                let negative_y = antenna1.1 > antenna2.1;

                let mut coord = if negative_x == negative_y {
                    (antenna1.0 - x_diff, antenna1.1 - y_diff)
                } else {
                    (antenna1.0 - x_diff, antenna1.1 + y_diff)
                };

                while coord.0 < grid.len() && coord.1 < grid[0].len() {
                    if coord != *antenna1 && coord != *antenna2 {
                        locations.insert(coord);
                        break;
                    }
                    coord = if negative_x == negative_y {
                        (coord.0 - x_diff, coord.1 - y_diff)
                    } else {
                        (coord.0 - x_diff, coord.1 + y_diff)
                    };
                }

                coord = if negative_x == negative_y {
                    (antenna1.0 + x_diff, antenna1.1 + y_diff)
                } else {
                    (antenna1.0 + x_diff, antenna1.1 - y_diff)
                };

                while coord.0 < grid.len() && coord.1 < grid[0].len() {
                    if coord != *antenna1 && coord != *antenna2 {
                        locations.insert(coord);
                        break;
                    }
                    coord = if negative_x == negative_y {
                        (coord.0 + x_diff, coord.1 + y_diff)
                    } else {
                        (coord.0 + x_diff, coord.1 - y_diff)
                    };
                }
            }
        }
    }

    println!("{}", locations.len());
}

fn part_two(input: &str) {
    let mut frequencies = HashSet::new();
    input.lines().for_each(|line| {
        line.chars().for_each(|c| {
            frequencies.insert(c);
        });
    });
    frequencies.remove(&'.');

    let mut locations = HashSet::new();

    for frequency in frequencies.iter() {
        let grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|x| if x != *frequency { '.' } else { *frequency })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut antennas = HashSet::new();
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == *frequency {
                    antennas.insert((i, j));
                }
            }
        }

        for antenna1 in antennas.iter() {
            for antenna2 in antennas.iter() {
                if *antenna1 == *antenna2 {
                    continue;
                }

                let x_diff = antenna1.0.abs_diff(antenna2.0);
                let y_diff = antenna1.1.abs_diff(antenna2.1);
                let negative_x = antenna1.0 > antenna2.0;
                let negative_y = antenna1.1 > antenna2.1;

                let mut coord = if negative_x == negative_y {
                    (antenna1.0 - x_diff, antenna1.1 - y_diff)
                } else {
                    (antenna1.0 - x_diff, antenna1.1 + y_diff)
                };

                while coord.0 < grid.len() && coord.1 < grid[0].len() {
                    locations.insert(coord);
                    coord = if negative_x == negative_y {
                        (coord.0 - x_diff, coord.1 - y_diff)
                    } else {
                        (coord.0 - x_diff, coord.1 + y_diff)
                    };
                }

                coord = if negative_x == negative_y {
                    (antenna1.0 + x_diff, antenna1.1 + y_diff)
                } else {
                    (antenna1.0 + x_diff, antenna1.1 - y_diff)
                };

                while coord.0 < grid.len() && coord.1 < grid[0].len() {
                    locations.insert(coord);
                    coord = if negative_x == negative_y {
                        (coord.0 + x_diff, coord.1 + y_diff)
                    } else {
                        (coord.0 + x_diff, coord.1 - y_diff)
                    };
                }
            }
        }
    }

    println!("{}", locations.len());
}
