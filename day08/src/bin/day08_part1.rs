use std::time::Instant;
use day08::{get_moves_to_solve, parse_input};

fn main() {
    let input = include_str!("../../resources/input.txt");
    let now = Instant::now();
    let map = parse_input(input);
    let result = get_moves_to_solve(&map);
    let elapsed = now.elapsed();
    println!("Result: {}, Elapsed: {:?}", result, elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test_input_01() {

        let input = include_str!("../../resources/test_input01.txt");
        let map = parse_input(input);
        let result = get_moves_to_solve(&map);
        assert_eq!(2, result);
    }

    #[test]
    fn run_test_input_02() {

        let input = include_str!("../../resources/test_input02.txt");
        let map = parse_input(input);
        let result = get_moves_to_solve(&map);
        assert_eq!(6, result);
    }
}
