use std::time::Instant;
use anyhow::Result;
use day08::{get_moves_to_solve, parse_input};

fn main() -> Result<()> {
    let input = include_str!("../../resources/input.txt");
    let now = Instant::now();
    let map = parse_input(input)?;
    let result = get_moves_to_solve(&map)?;
    let elapsed = now.elapsed();
    println!("Result: {}, Elapsed: {:?}", result, elapsed);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test_input_01() -> Result<()>{

        let input = include_str!("../../resources/test_input01.txt");
        let map = parse_input(input)?;
        println!("Map: {:?}", map);
        let result = get_moves_to_solve(&map)?;
        assert_eq!(2, result);
        Ok(())
    }

    #[test]
    fn run_test_input_02() -> Result<()> {

        let input = include_str!("../../resources/test_input02.txt");
        let map = parse_input(input)?;
        println!("Map: {:?}", map);
        let result = get_moves_to_solve(&map)?;
        assert_eq!(6, result);
        Ok(())
    }
}
