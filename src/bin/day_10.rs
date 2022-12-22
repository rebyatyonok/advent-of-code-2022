use advent_of_code_2022::get_input_file;
use std::collections::HashMap;

#[derive(Debug)]
enum Command {
    Add(i32),
    Noop,
}

fn line_to_command(line: &str) -> Command {
    let mut tokens = line.split_whitespace();
    let command = tokens.next().unwrap();

    match command {
        "addx" => {
            let amount = tokens.next().unwrap();
            Command::Add(amount.parse::<i32>().unwrap())
        }
        "noop" => Command::Noop,
        _ => panic!("Command unknown {}", command),
    }
}

fn first_task(file: &str) -> i32 {
    let mut register = 1;
    let mut cycle = 1;
    let breakpoints = vec![20, 60, 100, 140, 180, 220];
    let mut breakpoints_vals = breakpoints.iter().fold(HashMap::new(), |mut acc, e| {
        acc.entry(e).or_insert(0);
        acc
    });
    let mut current_breakpoint_index = 0;

    file.lines().map(line_to_command).for_each(|cmd| {
        let current_breakpoint = breakpoints.get(current_breakpoint_index);

        if let Some(val) = current_breakpoint {
            if cycle <= *val {
                breakpoints_vals.entry(val).and_modify(|e| *e = register);
            } else {
                current_breakpoint_index += 1;
            }
        }

        match cmd {
            Command::Add(amount) => {
                cycle += 2;
                register += amount;
            }
            Command::Noop => cycle += 1,
        }
    });

    breakpoints_vals
        .into_iter()
        .map(|(breakpoint, value)| breakpoint * value)
        .sum()
}

fn main() {
    let file = get_input_file();

    let first = first_task(&file);
    println!("{first}");
}
