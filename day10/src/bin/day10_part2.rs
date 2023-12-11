use std::time::Instant;
use anyhow::Result;
use day10::{get_encased_cells_count, parse_input};

fn main() -> Result<()>{
    let input = include_str!("../../resources/input.txt");
    let now = Instant::now();
    let tiles = parse_input(input)?;
    let result = get_encased_cells_count(&tiles)?;
    let elapsed = now.elapsed();
    println!("Result: {}, Elapsed: {:?}", result, elapsed);
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test_input04() -> Result<()>{

        let input = include_str!("../../resources/test_input04.txt");
        let tiles = parse_input(input)?;
        let result = get_encased_cells_count(&tiles)?;
        assert_eq!(result, 4);
        Ok(())
    }

    #[test]
    fn run_test_input05() -> Result<()>{

        let input = include_str!("../../resources/test_input05.txt");
        let tiles = parse_input(input)?;
        let result = get_encased_cells_count(&tiles)?;
        assert_eq!(result, 8);
        Ok(())
    }


    #[test]
    fn run_test_input06() -> Result<()>{

        let input = include_str!("../../resources/test_input06.txt");
        let tiles = parse_input(input)?;
        let result = get_encased_cells_count(&tiles)?;
        assert_eq!(result, 10);
        Ok(())
    }
}
