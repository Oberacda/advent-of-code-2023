use std::time::Instant;
use anyhow::Result;
use day10::{get_farthest_tile_in_loop_distance, parse_input};

fn main() -> Result<()>{
    let input = include_str!("../../resources/input.txt");
    let now = Instant::now();
    let tiles = parse_input(input)?;
    println!("{}", tiles);

    let result = get_farthest_tile_in_loop_distance(&tiles)?;
    let elapsed = now.elapsed();
    println!("Result: {}, Elapsed: {:?}", result, elapsed);
    Ok(())
}

#[cfg(test)]
mod tests {
    use day10::get_farthest_tile_in_loop_distance;
    use super::*;

    #[test]
    fn run_test_input01() -> Result<()>{

        let input = include_str!("../../resources/test_input01.txt");
        let tiles = parse_input(input)?;
        println!("{}", tiles);
        let result = get_farthest_tile_in_loop_distance(&tiles)?;
        assert_eq!(result, 4);
        Ok(())
    }

    #[test]
    fn run_test_input02() -> Result<()>{

        let input = include_str!("../../resources/test_input02.txt");
        let tiles = parse_input(input)?;
        println!("{}", tiles);
        let result = get_farthest_tile_in_loop_distance(&tiles)?;
        assert_eq!(result, 4);
        Ok(())
    }


    #[test]
    fn run_test_input03() -> Result<()>{

        let input = include_str!("../../resources/test_input03.txt");
        let tiles = parse_input(input)?;
        println!("{}", tiles);
        let result = get_farthest_tile_in_loop_distance(&tiles)?;
        assert_eq!(result, 8);
        Ok(())
    }
}
