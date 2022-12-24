use advent_of_code_2022::get_input_file;
use std::collections::HashMap;

#[derive(Debug)]
enum Command {
    Add(i32),
    Noop,
}

struct Crt {
    screen: Vec<Vec<char>>,
    pointer: (i32, i32),
}

impl Crt {
    fn new() -> Crt {
        let mut screen = vec![];
        for _ in 0..6 {
            let mut row = vec![];
            row.resize(40, ' ');
            screen.push(row);
        }
        Crt {
            screen,
            pointer: (0, 0),
        }
    }

    fn move_pointer_to_next(&mut self) {
        let (y, x) = self.pointer;

        if x >= 39 {
            self.pointer = (y + 1, 0);
        } else {
            self.pointer = (y, x + 1);
        }
    }

    fn set_lit_px(&mut self, x: i32, y: i32) {
        self.screen[y as usize][x as usize] = '#'
    }

    fn print_screen(&self) {
        self.screen
            .iter()
            .map(|line| line.iter().collect())
            .for_each(|line: String| println!("{:?}", line));
    }
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

fn second_task(file: &str) -> Crt {
    let mut skip = 0;
    let mut add = 0;
    let mut register = 1;
    let mut crt = Crt::new();
    let mut lines = file.lines();

    loop {
        let (y, x) = crt.pointer;

        if skip == 0 {
            register += add;
            add = 0;

            let line = lines.next();

            if line.is_none() {
                break;
            }

            match line_to_command(line.unwrap()) {
                Command::Add(amount) => {
                    skip += 2;
                    add += amount;
                }
                Command::Noop => skip += 1,
            }
        }

        if x == register || x == register - 1 || x == register + 1 {
            crt.set_lit_px(x, y);
        }

        skip -= 1;
        crt.move_pointer_to_next();
    }

    crt
}

fn main() {
    let file = get_input_file();

    let first = first_task(&file);
    println!("{first}");

    let crt = second_task(&file);
    crt.print_screen()
}
