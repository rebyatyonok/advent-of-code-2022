use std::{collections::HashSet, fs};

fn is_unique(string: &str) -> bool {
    let mut set = HashSet::new();

    for char in string.chars() {
        let result = set.insert(char);

        if !result {
            return result;
        }
    }

    true
}

fn find_marker_index(string: &String) -> i32 {
    let mut start_index = 0;

    for end_index in 4..string.len() {
        let marker = &string[start_index..end_index];

        if is_unique(marker) {
            return end_index as i32;
        } else {
            start_index += 1;
        }
    }

    start_index as i32
}

fn main() {
    let file = fs::read_to_string("day_06/src/input.txt").unwrap();

    println!("{}", find_marker_index(&file))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn all_correct() {
        assert_eq!(find_marker_index("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(find_marker_index("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(find_marker_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(find_marker_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}
