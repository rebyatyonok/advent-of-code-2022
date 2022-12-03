use std::fs;

#[derive(PartialEq, Debug)]
enum Shape {
    Rock,
    Scissors,
    Paper,
}

fn points_for_round(opponents: &Shape, yours: &Shape) -> i32 {
    if opponents.eq(&yours) {
        return 3;
    }

    if opponents.eq(&Shape::Rock) {
        if yours.eq(&Shape::Paper) {
            6
        } else {
            0
        }
    } else if opponents.eq(&Shape::Paper) {
        if yours.eq(&Shape::Scissors) {
            6
        } else {
            0
        }
    } else {
        if yours.eq(&Shape::Rock) {
            6
        } else {
            0
        }
    }
}

fn points_for_shape(shape: &Shape) -> i32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn get_shape_from_letter(letter: &str) -> Result<Shape, &'static str> {
    match letter {
        "A" | "X" => Ok(Shape::Rock),
        "B" | "Y" => Ok(Shape::Paper),
        "C" | "Z" => Ok(Shape::Scissors),
        _ => Err("Wrong letter!"),
    }
}

fn main() {
    let file = fs::read_to_string("rock_paper_scissors/src/input.txt").unwrap();
    let lines = file.lines();
    let mut result = 0;

    for line in lines {
        let line_of_shapes = line
            .split(" ")
            .map(|x| get_shape_from_letter(x).unwrap())
            .collect::<Vec<Shape>>();

        if let [opponents, yours] = &line_of_shapes[..] {
            result += points_for_round(&opponents, &yours) + points_for_shape(&yours);
        } else {
            panic!("Something went wrong!")
        }
    }

    println!("Result is {}", result)
}
