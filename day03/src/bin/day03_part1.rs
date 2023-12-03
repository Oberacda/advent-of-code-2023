use std::str::FromStr;

#[derive(Debug, Default)]
struct Number {
    y: usize,
    start_x: usize,
    end_x: usize,
    value: u64,
}

#[derive(Default, Debug)]
struct Symbol {
    line: usize,
    index: usize,
}

fn get_number_positions(input: &str) -> (Vec<Number>, Vec<Symbol>) {
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

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

            if char != '.' {
                symbols.push(Symbol {
                    line: line_index,
                    index,
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

    (numbers, symbols)
}

fn is_symbol_neighbor(number: &Number, symbol_covered_positions: &Vec<(usize, usize)>) -> bool {
    for idx in number.start_x..number.end_x + 1 {
        let index_covered = symbol_covered_positions.contains(&(idx, number.y));
        if !index_covered {
            continue;
        } else {
            return true;
        }
    }
    return false;
}

fn get_symbol_covered_positions(symbols: Vec<Symbol>) -> Vec<(usize, usize)> {
    symbols
        .iter()
        .flat_map(|x| {
            let mut results: Vec<(usize, usize)> = Vec::with_capacity(9);

            let opt_idx_n1 = usize::checked_sub(x.index, 1);
            let idx_0 = x.index;
            let idx_p1 = x.index + 1;

            let opt_idy_n1 = usize::checked_sub(x.line, 1);
            let idy_0 = x.line;
            let idy_p1 = x.line + 1;

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
        })
        .collect()
}

fn get_result(numbers: Vec<Number>, symbols: Vec<Symbol>) -> u64 {
    let symbol_positions = get_symbol_covered_positions(symbols);
    numbers
        .iter()
        .filter(|&x| is_symbol_neighbor(x, &symbol_positions))
        .map(|x| x.value)
        .sum::<u64>()
}

fn main() {
    let input = include_str!("../resources/input_01.txt");
    let (numbers, symbols) = get_number_positions(input);

    let result = get_result(numbers, symbols);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input01() {
        let input = include_str!("../resources/test_input_01.txt");
        let (numbers, symbols) = get_number_positions(input);
        assert_eq!(numbers.len(), 10);
        assert_eq!(symbols.len(), 6);

        let result = get_result(numbers, symbols);
        assert_eq!(result, 4361);
    }

    #[test]
    fn test_input02() {
        let input = include_str!("../resources/test_input_02.txt");
        let (numbers, symbols) = get_number_positions(input);
        let result = get_result(numbers, symbols);
        assert_eq!(result, 413);
    }

    #[test]
    fn test_input03() {
        let input = include_str!("../resources/test_input_03.txt");
        let (numbers, symbols) = get_number_positions(input);
        assert_eq!(symbols.len(), 9);
        assert_eq!(numbers.len(), 19);
        let result = get_result(numbers, symbols);
        assert_eq!(result, 925);
    }
}
