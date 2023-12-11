use std::time::Instant;
use anyhow::Result;
use day09::{get_result_path2, parse_input};

fn main() -> Result<()> {
    let input = include_str!("../../resources/input.txt");
    let now = Instant::now();
    let data = parse_input(input);
    let result = get_result_path2(&data)?;
    let elapsed = now.elapsed();
    println!("Result: {}, Elapsed: {:?}", result, elapsed);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test_input() -> Result<()>{
        let input = include_str!("../../resources/test_input.txt");
        let data = parse_input(input);
        let result = get_result_path2(&data)?;
        assert_eq!(2, result);
        Ok(())
    }
}
