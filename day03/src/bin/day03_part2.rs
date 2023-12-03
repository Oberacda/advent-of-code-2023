use std::str::FromStr;

#[derive(Debug, Default)]
struct Number {
    y: usize,
    start_x: usize,
    end_x: usize,
    value: u64,
}

#[derive(Default, Debug)]
struct Gear {
    y: usize,
    x: usize,
}

fn get_positions(input: &str) -> (Vec<Number>, Vec<Gear>) {
    let mut numbers: Vec<Number> = Vec::new();
    let mut gears: Vec<Gear> = Vec::new();

    let mut line_index: usize = 0;
    for line in input.lines() {
        let mut current_number_start_index: Option<usize> = None;
        let mut current_number_digits: String = String::new();
        for (index, char) in line.char_indices() {
            if char.is_ascii_digit() {
                if current_number_start_index.is_none() {
                    current_number_start_index = Some(index);
                }

                current_number_digits.push(char);
                continue;
            }

            if current_number_start_index.is_some() {
                let start_index = current_number_start_index.unwrap();
                let number = u64::from_str(current_number_digits.as_str()).unwrap();
                numbers.push(Number {
                    y: line_index,
                    start_x: start_index,
                    end_x: index - 1,
                    value: number,
                });
                current_number_start_index = None;
                current_number_digits = String::new();
            }

            if char == '*' {
                gears.push(Gear {
                    y: line_index,
                    x: index,
                });
            }
        }

        if current_number_start_index.is_some() {
            let start_index = current_number_start_index.unwrap();
            let number = u64::from_str(current_number_digits.as_str()).unwrap();
            numbers.push(Number {
                y: line_index,
                start_x: start_index,
                end_x: line.len() - 1,
                value: number,
            });
        }

        line_index += 1;
    }

    (numbers, gears)
}

fn is_gear_neighbor(number: &Number, gear_covered_positions: &Vec<(usize, usize)>) -> bool {
    for idx in number.start_x..number.end_x + 1 {
        let index_covered = gear_covered_positions.contains(&(idx, number.y));
        if !index_covered {
            continue;
        } else {
            return true;
        }
    }
    return false;
}

fn get_gear_covered_positions(gear: &Gear) -> Vec<(usize, usize)> {
    let mut results: Vec<(usize, usize)> = Vec::with_capacity(9);

    let opt_idx_n1 = usize::checked_sub(gear.x, 1);
    let idx_0 = gear.x;
    let idx_p1 = gear.x + 1;

    let opt_idy_n1 = usize::checked_sub(gear.y, 1);
    let idy_0 = gear.y;
    let idy_p1 = gear.y + 1;

    if opt_idy_n1.is_some() {
        let idy_n1 = opt_idy_n1.unwrap();
        results.extend_from_slice(&[(idx_0, idy_n1), (idx_p1, idy_n1)]);
    }
    if opt_idx_n1.is_some() {
        let idx_n1 = opt_idx_n1.unwrap();
        results.extend_from_slice(&[(idx_n1, idy_0), (idx_n1, idy_p1)]);
    }
    if opt_idx_n1.is_some() && opt_idy_n1.is_some() {
        let idy_n1 = opt_idy_n1.unwrap();
        let idx_n1 = opt_idx_n1.unwrap();
        results.push((idx_n1, idy_n1));
    }
    results.extend_from_slice(&[(idx_0, idy_p1), (idx_p1, idy_p1), (idx_p1, idy_0)]);

    results
}

fn get_result(numbers: Vec<Number>, gears: Vec<Gear>) -> u64 {
    let mut result: u64 = 0;
    for gear in gears {
        let gear_covered_positions = get_gear_covered_positions(&gear);
        let gear_values: Vec<u64> = numbers
            .iter()
            .filter(|&x| is_gear_neighbor(x, &gear_covered_positions))
            .map(|x| x.value)
            .collect();
        if gear_values.len() != 2 {
            continue;
        }
        result += gear_values.iter().product::<u64>();
    }
    result
}

fn main() {
    let input = include_str!("../../resources/input_01.txt");
    let (numbers, gears) = get_positions(input);

    let result = get_result(numbers, gears);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input01() {
        let input = include_str!("../../resources/test_input_01.txt");
        let (numbers, gears) = get_positions(input);
        assert_eq!(numbers.len(), 10);
        assert_eq!(gears.len(), 3);

        let result = get_result(numbers, gears);
        assert_eq!(result, 467835);
    }

    #[test]
    fn test_input02() {
        let input = include_str!("../../resources/test_input_02.txt");
        let (numbers, gears) = get_positions(input);
        let result = get_result(numbers, gears);
        assert_eq!(result, 6756);
    }

    #[test]
    fn test_input03() {
        let input = include_str!("../../resources/test_input_03.txt");
        let (numbers, gears) = get_positions(input);
        assert_eq!(gears.len(), 5);
        assert_eq!(numbers.len(), 19);
        let result = get_result(numbers, gears);
        assert_eq!(result, 6756);
    }
}
