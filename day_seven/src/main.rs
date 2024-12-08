fn main() {
    let input = include_str!("input.txt");
    part_two(input);
}

fn part_one(input: &str) {
    let mut sum = 0;

    input.lines().for_each(|line| {
        let mut colon_split = line.split(":");
        let total = colon_split.next().unwrap().parse::<u64>().unwrap();
        let mut nums = colon_split.next().unwrap().split(" ");
        nums.next();

        let mut results = vec![0];

        for num in nums.map(|num| num.parse::<u64>().unwrap()) {
            let mut new_results = vec![];
            for result in &results {
                let mult_result = result * num;
                let add_result = result + num;
                if mult_result <= total {
                    new_results.push(mult_result);
                }
                if add_result <= total {
                    new_results.push(add_result);
                }
            }
            results = new_results;
        }

        if results.contains(&total) {
            sum += total;
        }
    });

    println!("{}", sum);
}

fn part_two(input: &str) {
    let mut sum = 0;

    input.lines().for_each(|line| {
        let mut colon_split = line.split(":");
        let total = colon_split.next().unwrap().parse::<u64>().unwrap();
        let mut nums = colon_split.next().unwrap().split(" ");
        nums.next();

        let mut results = vec![0];

        for num in nums.map(|num| num.parse::<u64>().unwrap()) {
            let mut new_results = vec![];
            for result in &results {
                let mult_result = result * num;
                let add_result = result + num;
                let concat_result = format!("{}{}", result, num).parse::<u64>().unwrap();
                if mult_result <= total {
                    new_results.push(mult_result);
                }
                if add_result <= total {
                    new_results.push(add_result);
                }
                if concat_result <= total {
                    new_results.push(concat_result);
                }
            }
            results = new_results;
        }

        if results.contains(&total) {
            sum += total;
        }
    });

    println!("{}", sum);
}
