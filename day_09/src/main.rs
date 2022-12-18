use std::{cmp::Ordering, collections::HashSet};

#[derive(Debug, PartialEq, Eq)]
enum Motion {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

struct Rope {
    tail_motions: HashSet<Position>,
    head_pos: Position,
    tail_pos: Position,
    count: i32,
}

impl Rope {
    fn new() -> Rope {
        let tail_pos = Position { x: 0, y: 0 };
        Rope {
            tail_motions: HashSet::from([tail_pos]),
            head_pos: Position { x: 0, y: 0 },
            tail_pos,
            count: 0,
        }
    }

    fn move_head(&mut self, motion: &Motion, amount: i32) {
        for _ in 0..amount {
            self.count += 1;

            match motion {
                Motion::Up => self.head_pos.y += 1,
                Motion::Down => self.head_pos.y -= 1,
                Motion::Left => self.head_pos.x -= 1,
                Motion::Right => self.head_pos.x += 1,
            }

            if !self.are_ends_touching() {
                self.move_tail(motion);
            }
        }
    }

    fn move_tail(&mut self, head_motion: &Motion) {
        let mut new_position = self.tail_pos;

        match head_motion {
            Motion::Up | Motion::Down => {
                if *head_motion == Motion::Up {
                    new_position.y += 1;
                } else {
                    new_position.y -= 1;
                }

                match self.head_pos.x.cmp(&new_position.x) {
                    Ordering::Greater => new_position.x += 1,
                    Ordering::Less => new_position.x -= 1,
                    Ordering::Equal => (),
                }
            }
            Motion::Left | Motion::Right => {
                if *head_motion == Motion::Left {
                    new_position.x -= 1;
                } else {
                    new_position.x += 1;
                }

                match self.head_pos.y.cmp(&new_position.y) {
                    Ordering::Greater => new_position.y += 1,
                    Ordering::Less => new_position.y -= 1,
                    Ordering::Equal => (),
                }
            }
        }

        self.tail_pos = new_position;
        self.tail_motions.insert(new_position);
    }

    fn are_ends_touching(&self) -> bool {
        let dx = (self.head_pos.x - self.tail_pos.x).abs();
        let dy = (self.head_pos.y - self.tail_pos.y).abs();

        (dx <= 1) && (dy <= 1)
    }
}

fn line_to_motion_amount(line: &str) -> (Motion, i32) {
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
        .map(line_to_motion_amount)
        .for_each(|(motion, amount)| {
            rope.move_head(&motion, amount);
        });

    println!("{:?}", rope.tail_motions.len());
    println!("{:?}", rope.count);
}
