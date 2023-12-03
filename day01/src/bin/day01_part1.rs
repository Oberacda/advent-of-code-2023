use std::include_str;
use std::str::FromStr;

fn main() {
    let input = include_str!("../../resources/input01.txt");
    let input_lines: Vec<String> = input.lines().map(|x| x.to_string()).collect();

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

    println!("Sum: {}", line_values.iter().sum::<u64>());
}
