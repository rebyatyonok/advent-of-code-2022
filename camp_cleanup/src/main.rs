use std::fs;

fn contains(first: (i32, i32), second: (i32, i32)) -> bool {
    let (a, b) = first;
    let (c, d) = second;

    (a <= c && b >= d) || (c <= a && d >= b)
}

fn to_ranges(vec: &[i32]) -> ((i32, i32), (i32, i32)) {
    if vec.len() != 4 {
        panic!("Wrong length of parsed values!")
    } else {
        ((vec[0], vec[1]), (vec[2], vec[3]))
    }
}

fn first_task(file: &String) -> usize {
    file.lines()
        .filter(|line| {
            let parsed_line = line
                .split(',')
                .map(|range| range.split('-'))
                .flat_map(|x| x.map(|e| e.parse::<i32>().unwrap()))
                .collect::<Vec<i32>>();

            let (first, second) = to_ranges(&parsed_line);

            contains(first, second)
        })
        .count()
}

fn main() {
    let file = fs::read_to_string("camp_cleanup/src/input.txt").unwrap();

    println!("{}", first_task(&file))
}
