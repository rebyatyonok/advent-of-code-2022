use advent_of_code_2022::get_input_file;
use std::cmp;

fn main() {
    let file = get_input_file();

    println!("{}", most_calories(&file));
    println!("{:?}", top_three_sum(&file));
}

fn most_calories(file: &str) -> i32 {
    let mut most = 0;
    let mut current = 0;

    for line in file.lines() {
        if !line.is_empty() {
            current += line.parse::<i32>().unwrap_or(0);
        } else {
            most = cmp::max(most, current);
            current = 0;
        }
    }

    most
}

fn top_three_sum(file: &str) -> i32 {
    let mut sum = 0;
    let mut result = vec![0, 0, 0];

    file.lines().for_each(|line| {
        if !line.is_empty() {
            sum += line.parse::<i32>().unwrap_or(0);
        } else {
            result = result
                .iter()
                .map(|x| {
                    if sum > *x {
                        let tmp = sum;
                        sum = 0;
                        tmp
                    } else {
                        *x
                    }
                })
                .collect();

            sum = 0
        }
    });

    result.iter().sum()
}
