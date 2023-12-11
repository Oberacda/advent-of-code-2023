use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use anyhow::Result;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum PuzzleError <'a> {
    LogicError {
        msg: & 'a str
    }
}

impl <'a> Display for PuzzleError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to solve puzzle: ").ok();
        match self {
            PuzzleError::LogicError { msg } => {
                write!(f, "{}", msg)
            }
        }
    }
}

impl <'a> Error for PuzzleError <'a> {}

pub fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input.lines().map(|x| x.split_whitespace().filter_map(|x| i64::from_str(x).ok()).collect()).collect()
}

pub fn get_result_path1(data: &Vec<Vec<i64>>) -> Result<i64> {
    let mut results = Vec::new();
    for start_data_sequence in data {
        let mut data_sequences = Vec::new();
        data_sequences.push(Box::new(start_data_sequence.clone()));
        for i in 0..start_data_sequence.len() {
            let current_data_sequence = data_sequences.get(i).ok_or(PuzzleError::LogicError { msg: "Could not find data sequence!"})?;
            let next_data_sequence: Box<Vec<i64>> = Box::new(current_data_sequence.iter().zip(&current_data_sequence[1..]).map(|(x, y)| y - x).collect());
            let result = next_data_sequence.iter().all(|x| x.eq(&0_i64));

            data_sequences.push(next_data_sequence);
            if result {
                break;
            }
        }

        let mut prediction= 0_i64;

        for sequence in data_sequences.iter().rev() {
            let mut sequence_iter = sequence.iter();
            let last = sequence_iter.next_back().ok_or(PuzzleError::LogicError {msg: "Last sequence element missing!"})?;
            prediction += last;
        }
        results.push(prediction);
    }
    Ok(results.iter().sum::<i64>())
}

pub fn get_result_path2(data: &Vec<Vec<i64>>) -> Result<i64> {
    let mut results = Vec::new();
    for start_data_sequence in data {
        let mut data_sequences = Vec::new();
        data_sequences.push(Box::new(start_data_sequence.clone()));
        for i in 0..start_data_sequence.len() {
            let current_data_sequence = data_sequences.get(i).ok_or(PuzzleError::LogicError { msg: "Could not find data sequence!"})?;
            let next_data_sequence: Box<Vec<i64>> = Box::new(current_data_sequence.iter().zip(&current_data_sequence[1..]).map(|(x, y)| y - x).collect());
            let result = next_data_sequence.iter().all(|x| x.eq(&0_i64));

            data_sequences.push(next_data_sequence);
            if result {
                break;
            }
        }

        let mut prediction= 0_i64;

        for sequence in data_sequences.iter().rev() {
            let mut sequence_iter = sequence.iter();
            let last = sequence_iter.next().ok_or(PuzzleError::LogicError {msg: "First sequence element missing!"})?;
            prediction = last - prediction;
        }
        results.push(prediction);
    }
    Ok(results.iter().sum::<i64>())
}
