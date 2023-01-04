use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    hash::Hash,
};

#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
struct Point {
    x: usize,
    y: usize,
    priority: i32,
    value: u8,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Point {
    fn new(x: usize, y: usize, char: char) -> Self {
        Point {
            x,
            y,
            priority: 0,
            value: char_to_value(char),
        }
    }
}

struct Grid {
    grid: Vec<Vec<Point>>,
}

impl Grid {
    fn new(file: String) -> Grid {
        let grid = file
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, char)| Point::new(x, y, char))
                    .collect()
            })
            .collect();

        Grid { grid }
    }

    fn find(&self, char: char) -> Option<Point> {
        for row in self.grid.iter() {
            for point in row.iter() {
                if point.value == char_to_value(char) {
                    return Some(*point);
                }
            }
        }

        None
    }

    fn get_start_position(&self) -> Point {
        self.find('S').unwrap()
    }

    fn get_end_position(&self) -> Point {
        self.find('E').unwrap()
    }

    fn get_neighbors(&self, point: &Point) -> Vec<Point> {
        let x = point.x;
        let y = point.y;
        let mut variants = vec![(x, y + 1), (x + 1, y)];

        if y > 0 {
            variants.push((x, y - 1));
        }
        if x > 0 {
            variants.push((x - 1, y));
        }

        variants
            .into_iter()
            .filter_map(|(x, y)| match self.grid.get(y) {
                None => None,
                Some(row) => row.get(x).copied(),
            })
            .collect()
    }
}

fn get_path(start: &Point, end: &Point, grid: &Grid) -> Vec<Point> {
    let mut came_from = HashMap::new();
    let mut frontier = BinaryHeap::new();
    let mut visited: HashSet<Point> = HashSet::new();

    frontier.push(*start);
    came_from.insert(*start, None);

    while let Some(current) = frontier.pop() {
        let neighbors = grid.get_neighbors(&current);

        for next in neighbors.into_iter() {
            let is_visited = visited.get(&next).is_some();
            let is_value_ok = if next >= current {
                next.value.abs_diff(current.value) <= 1
            } else {
                true
            };

            if !is_visited && is_value_ok {
                frontier.push(next);
                came_from.insert(next, Some(current));
                visited.insert(next);
            }
        }
    }

    let mut current = *end;
    let mut path = vec![];

    while current != *start {
        path.push(current);
        current = came_from[&current].unwrap();
    }

    path
}

fn main() {
    let file = advent_of_code_2022::get_input_file();
    let grid = Grid::new(file);

    let start = grid.get_start_position();
    let end = grid.get_end_position();

    let path = get_path(&start, &end, &grid);

    println!("{:?}", path.len());
}

fn char_to_value(char: char) -> u8 {
    match char {
        'S' => 0,
        'E' => 27,
        char => char as u8 - 96,
    }
}
