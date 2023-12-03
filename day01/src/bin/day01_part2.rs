use std::include_str;
use std::str::FromStr;

fn calculate_number(input: &str) -> u64 {
    let input_lines: Vec<String> = input
        .lines()
        .map(|x| x.to_string())
        //.map(remove_text_digits)
        .map(|x| x.replace("one", "o1e"))
        .map(|x| x.replace("two", "t2o"))
        .map(|x| x.replace("three", "t3e"))
        .map(|x| x.replace("four", "f4r"))
        .map(|x| x.replace("five", "f5e"))
        .map(|x| x.replace("six", "s6x"))
        .map(|x| x.replace("seven", "s7n"))
        .map(|x| x.replace("eight", "e8t"))
        .map(|x| x.replace("nine", "n9e"))
        .collect();

    let mut line_values = Vec::<u64>::new();

    for input_line in input_lines {
        let mut digits = input_line.chars().filter(|x| x.is_ascii_digit());
        let first_numeric_char = digits.next().unwrap();
        let last_numeric_char = match digits.nth_back(0) {
            Some(digit) => digit,
            None => first_numeric_char,
        };
        line_values.push(
            u64::from_str(format!("{}{}", first_numeric_char, last_numeric_char).as_str()).unwrap(),
        );
    }

    line_values.iter().sum::<u64>()
}

fn main() {
    let input = include_str!("../../resources/input01.txt");
    let value = calculate_number(input);
    println!("Sum: {}", value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        let input = include_str!("../../resources/test_input01.txt");
        let value = calculate_number(input);

        assert_eq!(value, 299);
    }
}
