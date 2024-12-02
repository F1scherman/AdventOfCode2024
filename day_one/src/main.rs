fn main() {
    let input = include_str!("example.txt");
    part_one(input);
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
