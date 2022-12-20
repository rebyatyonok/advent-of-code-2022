use std::{cmp::Ordering, collections::HashSet};

use advent_of_code_2022::get_input_file;

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
    knots: Vec<Position>,
}

impl Rope {
    fn new() -> Rope {
        let mut knots = Vec::new();
        knots.resize(10, Position { x: 0, y: 0 });

        Rope {
            tail_motions: HashSet::from([Position { x: 0, y: 0 }]),
            knots,
        }
    }

    fn move_head(&mut self, motion: &Motion, amount: i32) {
        for _ in 0..amount {
            match motion {
                Motion::Up => self.knots[0].y += 1,
                Motion::Down => self.knots[0].y -= 1,
                Motion::Left => self.knots[0].x -= 1,
                Motion::Right => self.knots[0].x += 1,
            }

            for knot in 1..self.knots.len() {
                if !self.are_knots_touching(knot - 1, knot) {
                    self.move_knot(knot);
                }
            }
        }
    }

    fn move_knot(&mut self, knot_idx: usize) {
        let previous_knot = self.knots[knot_idx - 1];
        let mut new_position = self.knots[knot_idx];

        let dx = previous_knot.x - new_position.x;
        let dy = previous_knot.y - new_position.y;

        match dx.cmp(&0) {
            Ordering::Greater => new_position.x += 1,
            Ordering::Less => new_position.x -= 1,
            Ordering::Equal => (),
        }

        match dy.cmp(&0) {
            Ordering::Greater => new_position.y += 1,
            Ordering::Less => new_position.y -= 1,
            Ordering::Equal => (),
        }

        self.knots[knot_idx] = new_position;

        if knot_idx == 9 {
            self.tail_motions.insert(new_position);
        }
    }

    fn are_knots_touching(&self, hi: usize, ti: usize) -> bool {
        let dx = (self.knots[hi].x - self.knots[ti].x).abs();
        let dy = (self.knots[hi].y - self.knots[ti].y).abs();

        (dx <= 1) && (dy <= 1)
    }
}

fn line_to_motion_with_amount(line: &str) -> (Motion, i32) {
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
    let file = get_input_file();
    let mut rope = Rope::new();

    file.lines()
        .map(line_to_motion_with_amount)
        .for_each(|(motion, amount)| {
            rope.move_head(&motion, amount);
        });

    println!("{:?}", rope.tail_motions.len());
}
