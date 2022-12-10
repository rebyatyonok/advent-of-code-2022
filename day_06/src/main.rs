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

fn find_unique_substring_index(string: &String, substring_length: usize) -> i32 {
    let mut start_index = 0;

    for end_index in substring_length..string.len() {
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

    let first_marker = find_unique_substring_index(&file, 4);
    println!("{}", first_marker);

    let first_message_marker = find_unique_substring_index(&file, 14);
    println!("{}", first_message_marker);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn all_correct() {
        assert_eq!(
            find_unique_substring_index(&String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"), 4),
            5
        );
        assert_eq!(
            find_unique_substring_index(&String::from("nppdvjthqldpwncqszvftbrmjlhg"), 4),
            6
        );
        assert_eq!(
            find_unique_substring_index(&String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 14),
            19
        );
        assert_eq!(
            find_unique_substring_index(&String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 14),
            26
        );
    }
}
