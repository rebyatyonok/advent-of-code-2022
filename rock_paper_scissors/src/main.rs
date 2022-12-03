use std::fs;
use std::str;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Shape {
    Rock,
    Scissors,
    Paper,
}

enum MatchResult {
    Win,
    Loose,
    Draw,
}

fn get_win_shape_for(shape: &Shape) -> Shape {
    match shape {
        Shape::Paper => Shape::Scissors,
        Shape::Rock => Shape::Paper,
        Shape::Scissors => Shape::Rock,
    }
}

fn get_loose_shape_for(shape: &Shape) -> Shape {
    match shape {
        Shape::Paper => Shape::Rock,
        Shape::Rock => Shape::Scissors,
        Shape::Scissors => Shape::Paper,
    }
}

fn get_match_result(opponents: &Shape, yours: &Shape) -> MatchResult {
    if yours == &get_win_shape_for(opponents) {
        MatchResult::Win
    } else if yours == &get_loose_shape_for(opponents) {
        MatchResult::Loose
    } else {
        MatchResult::Draw
    }
}

fn get_points_for_round(opponents: &Shape, yours: &Shape) -> i32 {
    match get_match_result(opponents, yours) {
        MatchResult::Draw => 3,
        MatchResult::Loose => 0,
        MatchResult::Win => 6,
    }
}

fn get_points_for_shape(shape: &Shape) -> i32 {
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

fn get_match_result_from_letter(letter: &str) -> Result<MatchResult, &'static str> {
    match letter {
        "X" => Ok(MatchResult::Loose),
        "Y" => Ok(MatchResult::Draw),
        "Z" => Ok(MatchResult::Win),
        _ => Err("Wrong letter!"),
    }
}

fn main() {
    let file = fs::read_to_string("rock_paper_scissors/src/input.txt").unwrap();

    println!("First result is {}", first_task(&file));
    println!("Second result is {}", second_task(&file));
}

fn first_task(file: &String) -> i32 {
    let mut result = 0;

    for line in file.lines() {
        let line_of_shapes = line
            .split(" ")
            .map(|x| get_shape_from_letter(x).unwrap())
            .collect::<Vec<Shape>>();

        if let [opponents, yours] = &line_of_shapes[..] {
            result += get_points_for_round(&opponents, &yours) + get_points_for_shape(&yours);
        } else {
            panic!("Something went wrong!")
        }
    }

    result
}

fn second_task(file: &String) -> i32 {
    let mut result = 0;

    for line in file.lines() {
        let letters = line.split(" ").collect::<Vec<&str>>();
        let opponents = get_shape_from_letter(letters[0]).unwrap();
        let expected_result = get_match_result_from_letter(letters[1]).unwrap();

        let players = match expected_result {
            MatchResult::Win => get_win_shape_for(&opponents),
            MatchResult::Loose => get_loose_shape_for(&opponents),
            MatchResult::Draw => opponents,
        };

        result += get_points_for_round(&opponents, &players) + get_points_for_shape(&players);
    }

    result
}
