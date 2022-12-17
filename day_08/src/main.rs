use day_08::{get_are_trees_visible, to_matrix};

type Matrix<T = bool> = Vec<Vec<T>>;

fn get_horizontal_view_matrix(matrix: &Matrix<u32>) -> Matrix {
    let mut result = vec![];

    for line in matrix.iter() {
        result.push(get_are_trees_visible(line));
    }

    result
}

fn get_vertical_view_matrix(matrix: &Matrix<u32>) -> Matrix {
    let mut result: Matrix = matrix.iter().map(|_| vec![]).collect();
    let line_length = matrix[0].len();

    for i in 0..line_length {
        let column: Vec<u32> = matrix.iter().map(|line| line[i]).collect();
        get_are_trees_visible(&column)
            .iter()
            .enumerate()
            .for_each(|(line_i, res)| result[line_i].push(*res))
    }

    result
}

fn get_total_visibility(horizontal: Matrix, vertical: Matrix) -> Matrix {
    let mut result: Matrix = vec![];

    if horizontal.len() != vertical.len() {
        panic!("Different sizes!")
    }

    for i in 0..horizontal.len() {
        result.push(vec![]);

        for j in 0..horizontal[i].len() {
            result[i].push(horizontal[i][j] || vertical[i][j]);
        }
    }

    result
}

fn get_visibility_for_tree(matrix: &Vec<Vec<u32>>, position: (usize, usize)) -> i32 {
    let (y, x) = position;
    let current = matrix[y][x];

    if x == 0 || y == 0 {
        return 0;
    }

    // to right
    let mut right = 0;
    let mut right_x = x + 1;
    while right_x < matrix.first().expect("Can't take first").len() {
        right += 1;

        if matrix[y][right_x] >= current {
            break;
        } else {
            right_x += 1;
        }
    }

    // to left
    let mut left = 0;
    let mut left_x = x - 1;
    loop {
        left += 1;

        if matrix[y][left_x] >= current {
            break;
        }

        if left_x > 0 {
            left_x -= 1;
        } else {
            break;
        }
    }

    // up
    let mut up = 0;
    let mut up_y = y - 1;
    loop {
        up += 1;

        if matrix[up_y][x] >= current {
            break;
        }

        if up_y > 0 {
            up_y -= 1;
        } else {
            break;
        }
    }

    // down
    let mut down = 0;
    let mut down_y = y + 1;
    while down_y < matrix.len() {
        down += 1;

        if matrix[down_y][x] >= current {
            break;
        }

        down_y += 1;
    }

    right * left * up * down
}

fn main() {
    let file = std::fs::read_to_string("day_08/src/input.txt").unwrap();
    let matrix = to_matrix(file);
    let horizontal_visibility = get_horizontal_view_matrix(&matrix);
    let vertical_visibility = get_vertical_view_matrix(&matrix);
    let total_visibility = get_total_visibility(horizontal_visibility, vertical_visibility);

    let first_task: usize = total_visibility
        .iter()
        .map(|line| line.iter().filter(|is_visible| **is_visible).count())
        .sum();

    let mut visibilities = vec![];

    for (y, line) in matrix.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            visibilities.push(get_visibility_for_tree(&matrix, (y, x)));
        }
    }

    println!("{:?}", visibilities.iter().max());

    println!("{}", first_task);
}
