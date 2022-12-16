pub fn get_are_trees_visible(line: &Vec<u32>) -> Vec<bool> {
    let mut result: Vec<bool> = Vec::with_capacity(line.len());
    result.resize(line.len(), false);

    for (i, h) in line.iter().enumerate() {
        let height = *h;
        let max_start = *line[..i].iter().max().unwrap_or(&0);
        let max_end = *line[i + 1..].iter().max().unwrap_or(&0);

        let visible_from_start = height > max_start;
        let visible_from_end = height > max_end;

        if i == 0 || i == line.len() - 1 {
            result[i] = true;
            continue;
        }

        if visible_from_end || visible_from_start {
            result[i] = true;
        }
    }

    result
}

pub fn to_matrix(file: String) -> Vec<Vec<u32>> {
    file.lines()
        .into_iter()
        .map(|line| {
            line.chars()
                .into_iter()
                .map(|h| match h.to_digit(10) {
                    Some(num) => num,
                    None => panic!("Can not parse num {h:?}"),
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod day_09_tests {
    use crate::*;

    #[test]
    fn matrix_basic_case() {
        let input = String::from("123\n123\n123");
        let output = vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]];

        assert_eq!(to_matrix(input), output);
    }

    #[test]
    fn basic_case() {
        assert_eq!(
            get_are_trees_visible(&vec![2, 5, 5, 1, 2]),
            vec![true, true, true, false, true]
        );
    }

    #[test]
    fn invisible_between_visible() {
        assert_eq!(
            get_are_trees_visible(&vec![6, 5, 4, 3, 2, 6]),
            vec![true, false, false, false, false, true]
        );
    }

    #[test]
    fn highest_in_center() {
        assert_eq!(
            get_are_trees_visible(&vec![1, 2, 3, 4, 3, 2, 1]),
            vec![true, true, true, true, true, true, true]
        );
    }

    #[test]
    fn hightest_in_the_start() {
        assert_eq!(
            get_are_trees_visible(&vec![6, 5, 4, 3, 2, 1]),
            vec![true, true, true, true, true, true]
        );
    }

    #[test]
    fn starts_with_zero() {
        assert_eq!(
            get_are_trees_visible(&vec![0, 5, 5, 3, 5]),
            vec![true, true, false, false, true]
        );
    }

    #[test]
    fn hightest_in_the_end() {
        assert_eq!(
            get_are_trees_visible(&vec![2, 3, 3, 4, 5, 6]),
            vec![true, true, false, true, true, true]
        );
    }
}
