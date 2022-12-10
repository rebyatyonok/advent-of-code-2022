use std::fs;

type StacksState = Vec<Vec<char>>;
struct Command {
    from: usize,
    to: usize,
    amount: i32,
}

fn parse_crates(lines: Vec<&str>) -> StacksState {
    let mut crates_by_stacks: StacksState = Vec::new();

    for line in lines {
        let chars = line.chars().collect::<Vec<char>>();

        chars.chunks(4).enumerate().for_each(|(i, e)| {
            if crates_by_stacks.get(i) == None {
                crates_by_stacks.insert(i, Vec::new());
            };

            let stack = crates_by_stacks.get_mut(i).unwrap();

            if e[1] != ' ' {
                stack.insert(0, e[1])
            }
        });
    }

    crates_by_stacks
}

fn parse_commands(lines: Vec<&str>) -> Vec<Command> {
    lines
        .into_iter()
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<_>>();

            println!("{:?}", parts);

            Command {
                from: parts[3].parse::<usize>().unwrap() - 1,
                to: parts[5].parse::<usize>().unwrap() - 1,
                amount: parts[1].parse::<i32>().unwrap(),
            }
        })
        .collect::<Vec<_>>()
}

fn move_crate(from: &mut Vec<char>, to: &mut Vec<char>, amount: &mut i32) {
    while amount != &0 {
        match from.pop() {
            Some(char) => to.push(char),
            None => (),
        }

        *amount -= 1;
    }
}

fn first_task(state: &mut StacksState, commands: &mut Vec<Command>) -> String {
    commands.into_iter().for_each(|command| {
        let mut from = std::mem::take(&mut state[command.from]);
        let mut to = state.get_mut(command.to).unwrap();

        move_crate(&mut from, &mut to, &mut command.amount);

        state[command.from] = from;
    });

    state
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

fn main() {
    let file = fs::read_to_string("day_05/src/input.txt").unwrap();
    let mut initial_state_lines: Vec<&str> = Vec::new();
    let mut command_lines: Vec<&str> = Vec::new();

    for line in file.lines() {
        if line.contains('[') {
            initial_state_lines.push(line);
        }
        if line.contains("move") {
            command_lines.push(line);
        }
    }

    let mut initial_state = parse_crates(initial_state_lines);
    let mut commands = parse_commands(command_lines);

    println!("{}", first_task(&mut initial_state, &mut commands))
}
