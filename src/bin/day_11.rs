#[derive(Debug)]
enum Operation {
    Add(i32),
    Multiply(i32),
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

    fn exec(&self, item: i32) -> i32 {
        match self {
            Operation::Add(x) => item + x,
            Operation::Multiply(x) => item * x,
            Operation::Square => item * item,
        }
    }
}

#[derive(Debug)]
struct Test {
    value: i32,
    if_true: usize,
    if_false: usize,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i32>,
    operation: Operation,
    inspection_count: i32,
    test: Test,
}

impl Monkey {
    fn new(string: &str) -> Monkey {
        let mut lines = string.lines().skip(1);

        let mut next_line = || lines.next().expect("Expected line").trim();

        let items = next_line()
            .split_whitespace()
            .skip(2)
            .map(|x| x.trim_end_matches(',').parse::<i32>().unwrap())
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

    fn inspect(&mut self) -> (usize, i32) {
        self.inspection_count += 1;
        let item = self.items.remove(0);

        let inspected_item = self.get_value_after_inspection(item);

        let monkey_to_pass = if item % self.test.value == 0 {
            self.test.if_true
        } else {
            self.test.if_false
        };

        (monkey_to_pass, inspected_item)
    }

    fn get_value_after_inspection(&self, item: i32) -> i32 {
        (self.operation.exec(item) as f32 / 3_f32).floor() as i32
    }

    fn add_item(&mut self, item: i32) {
        self.items.push(item);
    }
}

fn main() {
    let file = advent_of_code_2022::get_input_file();
    let monkeys: Vec<Monkey> = file.split("\n\n").map(Monkey::new).collect();

    println!("{monkeys:#?}");
}
