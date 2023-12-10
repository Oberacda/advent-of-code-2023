use std::time::Instant;
use day07::{calculate_result, parse_input};

fn main() {
    let input = include_str!("../../resources/input.txt");
    let now = Instant::now();
    let mut hands = parse_input(input);
    let result = calculate_result(&mut hands);
    let elapsed = now.elapsed();
    println!("Result: {}, Elapsed: {:?}", result, elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {

        let input = include_str!("../../resources/test_input.txt");
        let mut hands = parse_input(input);
        println!("Hands: {:?}", hands);
        let result = calculate_result(&mut hands);
        assert_eq!(6440, result);
    }
}
