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

fn main() {
    let file = std::fs::read_to_string("day_08/src/input.txt").unwrap();
    let matrix = to_matrix(file);
    let horizontal_visibility = get_horizontal_view_matrix(&matrix);
    let vertical_visibility = get_vertical_view_matrix(&matrix);
    let total_visibility = get_total_visibility(horizontal_visibility, vertical_visibility);

    // println!("{:?}", vertical_visibility);

    let first_task: usize = total_visibility
        .iter()
        .map(|line| line.iter().filter(|is_visible| **is_visible).count())
        .sum();

    println!("{}", first_task);
}
