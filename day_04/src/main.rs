use std::fs;

type TRange = (i32, i32);
type TRanges = (TRange, TRange);

fn contains(first: &TRange, second: &TRange) -> bool {
    let (x1, x2) = first;
    let (y1, y2) = second;

    (x1 <= y1 && x2 >= y2) || (y1 <= x1 && y2 >= x2)
}

fn overlaps(first: &TRange, second: &TRange) -> bool {
    let (x1, x2) = first;
    let (y1, y2) = second;

    x1 <= y2 && y1 <= x2
}

fn to_ranges(vec: &[i32]) -> TRanges {
    if vec.len() != 4 {
        panic!("Wrong length of parsed values!")
    } else {
        ((vec[0], vec[1]), (vec[2], vec[3]))
    }
}

fn first_task(ranges: &Vec<TRanges>) -> usize {
    ranges
        .iter()
        .filter(|x| {
            let (f, s) = x;
            contains(f, s)
        })
        .count()
}

fn second_task(ranges: &Vec<TRanges>) -> usize {
    ranges
        .iter()
        .filter(|x| {
            let (f, s) = x;
            overlaps(f, s)
        })
        .count()
}

fn main() {
    let file = fs::read_to_string("day_04/src/input.txt").unwrap();

    let parsed_lines = file.lines().map(|line| {
        line.split(',')
            .map(|range| range.split('-'))
            .flat_map(|x| x.map(|e| e.parse::<i32>().unwrap()))
            .collect::<Vec<i32>>()
    });

    let lines_as_ranges = parsed_lines.map(|x| to_ranges(&x)).collect();

    println!("{}", first_task(&lines_as_ranges));
    println!("{}", second_task(&lines_as_ranges));
}
