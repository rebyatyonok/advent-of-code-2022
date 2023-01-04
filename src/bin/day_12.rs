use std::collections::HashSet;

type Visited = HashSet<String>;
type Matrix = Vec<Vec<char>>;

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "x:{} y:{}", self.x, self.y)
    }
}

fn is_valid(current: Point, next: Point, matrix: &Matrix, visited: &mut Visited) -> bool {
    let next_char = matrix[next.y][next.x];
    let current_char = matrix[current.y][current.x];

    let is_visited = visited.get(&next.to_string()).is_some();
    let is_next_letter_okay = if current_char == 'S' && next_char == 'a' {
        true
    } else if next_char > current_char {
        next_char as u8 == current_char as u8 + 1
    } else if next_char == 'E' {
        current_char == 'z'
    } else {
        true
    };

    is_next_letter_okay && !is_visited
}

fn find_path(
    current: Point,
    matrix: &Matrix,
    to: char,
    visited: &mut Visited,
    path: &mut Vec<char>,
) -> bool {
    let x = current.x;
    let y = current.y;

    visited.insert(current.to_string());

    if y >= matrix.len() || x >= matrix[0].len() {
        return false;
    }

    let mut next_coordinates = vec![];

    if x + 1 < matrix[0].len() {
        next_coordinates.push((x + 1, y))
    }
    if y + 1 < matrix.len() {
        next_coordinates.push((x, y + 1))
    }
    if x > 0 {
        next_coordinates.push((x - 1, y))
    }
    if y > 0 {
        next_coordinates.push((x, y - 1));
    }

    if matrix[current.y][current.x] == to {
        println!("Found E in {} {}", current.x, current.y);
        return true;
    }

    for next in next_coordinates {
        let point = Point {
            x: next.0,
            y: next.1,
        };

        if is_valid(current, point, matrix, visited) && find_path(point, matrix, to, visited, path)
        {
            path.push(matrix[current.y][current.x]);
            return true;
        }
    }

    false
}

fn find_char(matrix: &[Vec<char>], char: char) -> Option<Point> {
    for (y, line) in matrix.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            if matrix[y][x] == char {
                return Some(Point { x, y });
            }
        }
    }

    None
}

fn main() {
    let file = advent_of_code_2022::get_input_file();
    let matrix: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();
    let mut path = vec![];
    let mut visited = HashSet::new();

    let start = find_char(&matrix, 'S').expect("Expected to have an S char in input");

    find_path(start, &matrix, 'E', &mut visited, &mut path);
    println!("{:?}", visited);

    println!("{:?}", path.len());
}
