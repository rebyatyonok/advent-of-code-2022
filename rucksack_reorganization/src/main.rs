use std::{char, collections::HashSet, fs};

/// Get order for char in a task-specific range
fn get_order_num_for_char(char: char) -> i32 {
    if char.is_uppercase() {
        // we need to cast letters A - Z to range 27 - 52
        (char as i32) - 38
    } else {
        // we need to cast letters a-z to range 1 - 26
        (char as i32) - 96
    }
}

fn get_common_letter_for_halves(str: &str) -> Option<char> {
    let middle = str.len() / 2;
    let mut first_half: HashSet<char> = HashSet::new();

    for (i, char) in str.chars().enumerate() {
        if i < middle {
            first_half.insert(char);
        } else {
            if first_half.contains(&char) {
                return Some(char);
            }
        }
    }

    None
}

fn first_task(file: &String) -> i32 {
    let mut result = 0;

    for line in file.lines() {
        let same_letter = get_common_letter_for_halves(line).unwrap();
        result += get_order_num_for_char(same_letter);
    }

    result
}

fn main() {
    let file = fs::read_to_string("rucksack_reorganization/src/input.txt").unwrap();

    println!("{}", first_task(&file));
}

#[cfg(test)]
mod tests {
    use crate::{get_common_letter_for_halves, get_order_num_for_char};

    #[test]
    fn lowercase_letters_in_correct_range() {
        assert_eq!(get_order_num_for_char('a'), 1);
        assert_eq!(get_order_num_for_char('p'), 16);
        assert_eq!(get_order_num_for_char('v'), 22);
        assert_eq!(get_order_num_for_char('z'), 26);
    }

    #[test]
    fn uppercase_letters_in_correct_range() {
        assert_eq!(get_order_num_for_char('A'), 27);
        assert_eq!(get_order_num_for_char('L'), 38);
        assert_eq!(get_order_num_for_char('P'), 42);
        assert_eq!(get_order_num_for_char('Z'), 52);
    }

    #[test]
    fn common_letters_works() {
        let str = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        assert_eq!(get_common_letter_for_halves(str), Some('L'));
    }
}
