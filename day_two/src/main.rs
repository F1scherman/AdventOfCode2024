fn main() {
    let input = include_str!("input.txt");
    part_two(input);
}

fn part_one(input: &str) {
    let mut sum = 0;
    input.lines().for_each(|line| {
        let mut levels = line
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut sorted_levels = levels.clone();
        let mut reversed_levels = levels.clone();
        sorted_levels.sort();
        reversed_levels.reverse();
        if levels.eq(&sorted_levels) || reversed_levels.eq(&sorted_levels) {
            let mut works = true;
            for i in 1..levels.len() {
                let diff = (levels[i - 1] - levels[i]).abs();
                if 3 >= diff && diff >= 1 {
                    continue;
                } else {
                    works = false;
                    break;
                }
            }
            if works {
                sum += 1;
            }
        }
    });

    println!("{}", sum);
}

fn part_two(input: &str) {
    let mut sum = 0;
    input.lines().for_each(|line| {
        let mut levels_original = line
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        'outer: for i in 0..(levels_original.len() + 1) {
            let mut levels = levels_original.clone();
            if i != levels_original.len() {
                levels.remove(i);
            }
            let mut sorted_levels = levels.clone();
            let mut reversed_levels = levels.clone();
            sorted_levels.sort();
            reversed_levels.reverse();
            if levels.eq(&sorted_levels) || reversed_levels.eq(&sorted_levels) {
                let mut works = true;
                'inner: for i in 1..levels.len() {
                    let diff = (levels[i - 1] - levels[i]).abs();
                    if 3 >= diff && diff >= 1 {
                        continue;
                    } else {
                        works = false;
                        break 'inner;
                    }
                }
                if works {
                    sum += 1;
                    break 'outer;
                }
            }
        }
    });

    println!("{}", sum);
}
