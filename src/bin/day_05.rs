use advent_of_code_2022::get_input_file;

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
            if crates_by_stacks.get(i).is_none() {
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

            Command {
                from: parts[3].parse::<usize>().unwrap() - 1,
                to: parts[5].parse::<usize>().unwrap() - 1,
                amount: parts[1].parse::<i32>().unwrap(),
            }
        })
        .collect::<Vec<_>>()
}

fn move_crate(from: &mut Vec<char>, to: &mut Vec<char>, amount: &mut i32) {
    let split_off_index = from.len() - *amount as usize;
    let mut movable = from.split_off(split_off_index);
    to.append(&mut movable);
}

fn tasks(state: &mut StacksState, commands: &mut Vec<Command>) -> String {
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
    let file = get_input_file();
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

    println!("{}", tasks(&mut initial_state, &mut commands))
}
