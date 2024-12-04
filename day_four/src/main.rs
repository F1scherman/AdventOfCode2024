use regex::Regex;
use std::str;

fn main() {
    let input = include_str!("input.txt");
    part_two(input);
}

fn part_one(input: &str) {
    let mut sum = 0;

    let xmas_regex = Regex::new(r"XMAS").unwrap();
    let samx_regex = Regex::new(r"SAMX").unwrap();
    sum += xmas_regex.captures_iter(input).count(); // Horizontal
    sum += samx_regex.captures_iter(input).count();

    println!("{}", sum);

    let lines = input.lines().map(|x| x.as_bytes()).collect::<Vec<&[u8]>>();
    let mut columns = vec![vec![]; lines[0].len()];

    let x = "X".as_bytes()[0];
    let m = "M".as_bytes()[0];
    let a = "A".as_bytes()[0];
    let s = "S".as_bytes()[0];

    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            columns[j].push(lines[i][j]);
        }
    }

    sum += columns
        .iter()
        .map(|x| {
            xmas_regex
                .captures_iter(str::from_utf8(&x).unwrap())
                .count()
        })
        .sum::<usize>(); // Verticals
    sum += columns
        .iter()
        .map(|x| {
            samx_regex
                .captures_iter(str::from_utf8(&x).unwrap())
                .count()
        })
        .sum::<usize>();

    println!("{}", sum);

    for i in 0..(lines.len() - 3) {
        for j in 0..(lines[i].len() - 3) {
            if lines[i][j] == x {
                if lines[i + 1][j + 1] == m && lines[i + 2][j + 2] == a && lines[i + 3][j + 3] == s
                {
                    sum += 1;
                }
            } else if lines[i][j] == s {
                if lines[i + 1][j + 1] == a && lines[i + 2][j + 2] == m && lines[i + 3][j + 3] == x
                {
                    sum += 1;
                }
            }
        }
    }

    println!("{}", sum);

    for i in 0..(lines.len() - 3) {
        for j in 3..lines[i].len() {
            if lines[i][j] == x {
                if lines[i + 1][j - 1] == m && lines[i + 2][j - 2] == a && lines[i + 3][j - 3] == s
                {
                    sum += 1;
                }
            } else if lines[i][j] == s {
                if lines[i + 1][j - 1] == a && lines[i + 2][j - 2] == m && lines[i + 3][j - 3] == x
                {
                    sum += 1;
                }
            }
        }
    }

    println!("{}", sum);
}

fn part_two(input: &str) {
    let mut sum = 0;

    let lines = input.lines().map(|x| x.as_bytes()).collect::<Vec<&[u8]>>();

    let x = "X".as_bytes()[0];
    let m = "M".as_bytes()[0];
    let a = "A".as_bytes()[0];
    let s = "S".as_bytes()[0];

    for i in 1..(lines.len() - 1) {
        for j in 1..(lines[i].len() - 1) {
            if lines[i][j] != a {
                continue;
            }

            if lines[i - 1][j - 1] == x || lines[i - 1][j - 1] == a {
                continue;
            }
            if lines[i + 1][j - 1] == x || lines[i + 1][j - 1] == a {
                continue;
            }
            if lines[i - 1][j + 1] == x || lines[i - 1][j + 1] == a {
                continue;
            }
            if lines[i + 1][j + 1] == x || lines[i + 1][j + 1] == a {
                continue;
            }

            if lines[i - 1][j - 1] == m && lines[i + 1][j + 1] != s {
                continue;
            }
            if lines[i - 1][j - 1] == s && lines[i + 1][j + 1] != m {
                continue;
            }
            if lines[i + 1][j - 1] == m && lines[i - 1][j + 1] != s {
                continue;
            }
            if lines[i + 1][j - 1] == s && lines[i - 1][j + 1] != m {
                continue;
            }

            sum += 1;
        }
    }

    println!("{}", sum);
}
