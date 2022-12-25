#[derive(Debug)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl Operation {
    fn from_line(string: &str) -> Operation {
        let mut tokens = string.split_whitespace().skip(3);
        let _first_arg = tokens.next().unwrap();
        let operation = tokens.next().unwrap();
        let second_arg = tokens.next().unwrap();

        match operation {
            "+" => Operation::Add(second_arg.parse().unwrap()),
            "*" => match second_arg {
                "old" => Operation::Square,
                _ => Operation::Multiply(second_arg.parse().unwrap()),
            },
            _ => panic!("Unknown operation {operation:?}"),
        }
    }

    fn exec(&self, item: u64) -> u64 {
        match self {
            Operation::Add(x) => item + x,
            Operation::Multiply(x) => item * x,
            Operation::Square => item.pow(2),
        }
    }
}

#[derive(Debug)]
struct Test {
    value: u64,
    if_true: usize,
    if_false: usize,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    inspection_count: u64,
    test: Test,
}

impl Monkey {
    fn new(string: &str) -> Monkey {
        let mut lines = string.lines().skip(1);

        let mut next_line = || lines.next().expect("Expected line").trim();

        let items = next_line()
            .split_whitespace()
            .skip(2)
            .map(|x| x.trim_end_matches(',').parse::<u64>().unwrap())
            .collect();

        let operation = Operation::from_line(next_line());

        let test: Test = {
            let mut take_last_and_parse = || {
                next_line()
                    .split_whitespace()
                    .next_back()
                    .expect("Expected string")
                    .parse()
                    .expect("Expected number")
            };

            let value = take_last_and_parse();
            let if_true = take_last_and_parse() as usize;
            let if_false = take_last_and_parse() as usize;

            Test {
                value,
                if_true,
                if_false,
            }
        };

        Monkey {
            items,
            operation,
            inspection_count: 0,
            test,
        }
    }

    fn inspect(&mut self, item: u64, hcd: u64) -> (usize, u64) {
        self.inspection_count += 1;

        println!("{item:?}");
        let mut inspected_item = self.operation.exec(item);

        inspected_item %= hcd; // do not need that for second task

        let monkey_to_pass = if inspected_item % self.test.value == 0 {
            self.test.if_true
        } else {
            self.test.if_false
        };

        (monkey_to_pass, inspected_item)
    }

    fn add_items(&mut self, items: Option<&mut Vec<u64>>) {
        if let Some(items) = items {
            self.items.append(items);
        }
    }
}

fn main() {
    let file = advent_of_code_2022::get_input_file();
    let mut monkeys: Vec<Monkey> = file.split("\n\n").map(Monkey::new).collect();

    let mut items_in_the_air = std::collections::HashMap::<usize, Vec<u64>>::new();

    let hcd = monkeys.iter().fold(1, |acc, m| acc * m.test.value);

    for _ in 0..10000 {
        for (i, monkey) in monkeys.iter_mut().enumerate() {
            monkey.add_items(items_in_the_air.remove(&i).as_mut());

            while let Some(item) = monkey.items.pop() {
                let (j, item) = monkey.inspect(item, hcd);

                items_in_the_air.entry(j).or_default().push(item);
            }
        }
    }

    monkeys.sort_by(|m1, m2| m2.inspection_count.cmp(&m1.inspection_count));

    let first_task = monkeys
        .iter()
        .take(2)
        .fold(1, |acc, m| m.inspection_count * acc);

    println!("{first_task:#?}");
}
