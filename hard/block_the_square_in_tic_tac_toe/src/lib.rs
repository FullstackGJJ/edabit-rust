#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref WINNING_PATTERNS: Vec<Vec<i8>> = vec![
        // horizontal winning
        vec![0, 1, 2],
        vec![3, 4, 5],
        vec![6, 7, 8],

        // vertical winning
        vec![0, 3, 6],
        vec![1, 4, 7],
        vec![2, 5, 8],

        // diagonal winning
        vec![0, 4, 8],
        vec![2, 4, 6],
    ];
}

fn determine_blocking_position(input: (i8, i8)) -> i8 {
    // Find winnable that contains first and second point and determine overlapping remaining point needed
    let (first, second) = input;

    for path in WINNING_PATTERNS.iter() {
        if path.contains(&first) && path.contains(&second) {
            let blocking_point = path.into_iter().find(|&&x| x != first && x != second);
            return match blocking_point {
                Some(point) => *point,
                None => 10
            }
        }
    }

    10
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::*;

    // Assumes input as number in 3 x 3 grid populated from 0 - 8
    #[test_case((0, 3), 6)]
    #[test_case((0, 8), 4)]
    #[test_case((4, 8), 0)]
    #[test_case((2, 5), 8)]
    #[test_case((1, 7), 4)]
    #[test_case((0, 1), 2)]
    fn determine_blocking_position_tests(input: (i8, i8), expected_result: i8) {
        let result = determine_blocking_position(input);
        assert_eq!(expected_result, result);
    }
}
