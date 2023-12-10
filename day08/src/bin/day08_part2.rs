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
