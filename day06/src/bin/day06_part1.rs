use std::time::Instant;
use day06::{parse_input, get_winning_race_strategy_count};

fn main() {
    let input = include_str!("../../resources/input_part1.txt");
    let now = Instant::now();
    let races = parse_input(input);
    let result = get_winning_race_strategy_count(&races);
    let elapsed = now.elapsed();

    println!("Solution: {}; Elapsed: {:?}", result, elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../../resources/test_input01.txt");
        let races = parse_input(input);
        println!("Races: {:?}", races);
        let result = get_winning_race_strategy_count(&races);
        assert_eq!(288, result);
    }
}