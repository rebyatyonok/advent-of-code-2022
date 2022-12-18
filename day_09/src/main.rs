use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
enum Motion {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

struct Rope {
    tail_motions: Vec<Position>,
    head_pos: Position,
    tail_pos: Position,
}

impl Rope {
    fn new() -> Rope {
        Rope {
            tail_motions: vec![],
            head_pos: Position { x: 0, y: 0 },
            tail_pos: Position { x: 0, y: 0 },
        }
    }

    fn move_head(&mut self, motion: &Motion, amount: i32) {
        // println!("=== {:?} {} ===", direction, amount);
        for _ in 0..amount {
            match motion {
                Motion::Up => self.head_pos.y += 1,
                Motion::Down => self.head_pos.y -= 1,
                Motion::Left => self.head_pos.x -= 1,
                Motion::Right => self.head_pos.x += 1,
            }

            // println!("Head {:?}", self.head_pos);

            if !self.is_touching() {
                self.adjust_tail(motion);
            }
        }
    }

    fn adjust_tail(&mut self, head_motion: &Motion) {
        let mut new = self.tail_pos;

        match head_motion {
            Motion::Up | Motion::Down => {
                if *head_motion == Motion::Up {
                    new.y += 1;
                } else {
                    new.y -= 1;
                }

                match self.head_pos.x.cmp(&new.x) {
                    Ordering::Greater => new.x += 1,
                    Ordering::Less => new.x -= 1,
                    Ordering::Equal => (),
                }
            }
            Motion::Left | Motion::Right => {
                if *head_motion == Motion::Left {
                    new.x -= 1;
                } else {
                    new.x += 1;
                }

                match self.head_pos.y.cmp(&new.y) {
                    Ordering::Greater => new.y += 1,
                    Ordering::Less => new.y -= 1,
                    Ordering::Equal => (),
                }
            }
        }

        self.tail_pos = new;
        self.tail_motions.push(new);
    }

    fn is_touching(&self) -> bool {
        let x_diff = (self.head_pos.x - self.tail_pos.x).abs();
        let y_diff = (self.head_pos.y - self.tail_pos.y).abs();

        // println!("x_diff {} y_diff {}", x_diff, y_diff);

        (x_diff <= 1) && (y_diff <= 1)
    }
}

fn line_to_motion(line: &str) -> (Motion, i32) {
    let mut tokens = line.split_whitespace();
    let direction = tokens.next().expect("No direction!");
    let amount = tokens.next().expect("No amount").parse::<i32>().unwrap();

    let motion = match direction {
        "U" => Motion::Up,
        "D" => Motion::Down,
        "L" => Motion::Left,
        "R" => Motion::Right,
        _ => panic!("Unknown direction {}", direction),
    };

    (motion, amount)
}

fn main() {
    let file = std::fs::read_to_string("day_09/src/input.txt").unwrap();
    let mut rope = Rope::new();

    file.lines()
        .map(line_to_motion)
        .for_each(|(motion, amount)| {
            rope.move_head(&motion, amount);
        });

    println!("{:?}", rope.tail_motions.len());
}
