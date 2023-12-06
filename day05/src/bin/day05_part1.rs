use std::time::Instant;
use day05::{create_almanac, find_lowest_location_compression, parse_seeds_part1};

fn main() {
    let now = Instant::now();
    let input: Vec<&str> = include_str!("../../resources/input01.txt").split_terminator("\n\n").collect();
    let almanac = create_almanac(input, parse_seeds_part1);
    let res = find_lowest_location_compression(&almanac);
    println!("Result: {}", res);
    let elapsed = now.elapsed();
    println!("Elapsed: {:?}", elapsed);
}

#[cfg(test)]
mod tests {
    use day05::find_lowest_location_compression;
    use super::*;

    #[test]
    fn test_input01() {

        let input: Vec<&str> = include_str!("../../resources/test_input01.txt").split_terminator("\n\n").collect();
        let almanac = create_almanac(input, parse_seeds_part1);
        let res = find_lowest_location_compression(&almanac);
        assert_eq!(res, 35);
    }
}
