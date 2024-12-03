use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    part_two(input);
}

fn part_one(input: &str) {
    let instruction_regex = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let values_regex = Regex::new(r"[0-9]+").unwrap();
    let mut sum = 0;

    for instruction in instruction_regex.captures_iter(input).map(|c| c.extract::<0>().0) {
        let values = values_regex.captures_iter(instruction).map(|c| c.extract::<0>().0.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        sum += values[0] * values[1];
    }

    println!("{}", sum);
}

fn part_two(input: &str) {
    let instruction_regex = Regex::new(r"mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    let values_regex = Regex::new(r"[0-9]+").unwrap();
    let mut sum = 0;

    for instruction in instruction_regex.captures_iter(input).map(|c| c.extract::<0>().0) {
        if instruction.to_string() == "do()" {
            enabled = true;
        } else if instruction.to_string() == "don't()" {
            enabled = false;
        } else if enabled {
            let values = values_regex.captures_iter(instruction).map(|c| c.extract::<0>().0.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            sum += values[0] * values[1];
        }
    }

    println!("{}", sum);
}
