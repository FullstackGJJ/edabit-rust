fn determine_blocking_position(input: (i8, i8)) -> i8 {
    // Find winnable paths for first and second point and determine overlapping remaining point needed
    let (first, second) = input;

}

fn determine_remaining_points_for_win(point: i8) -> Vec<pair<i8>> {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterate_determine_remaining_points_for_win_input() {
        let inputs = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let expected_value_map = [
            (0, vec![(1, 2)])
        ];
    }

    // Assumes input as number in 3 x 3 grid populated from 0 - 8
    #[test_case((0, 3), 6)]
    //#[test_case((0, 8), 4)]
    //#[test_case((4, 8), 0)]
    //#[test_case((2, 5), 8)]
    use test_case::test_case;
    //#[test_case((1, 7), 4)]
    //#[test_case((0, 1), 2)]
    fn it_works(input: (i8, i8), expected_result: i8) {
        let result = determine_blocking_position(input);
        assert_eq!(expected_result, result);
    }
}
