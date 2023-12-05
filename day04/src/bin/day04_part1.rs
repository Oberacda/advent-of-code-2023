use std::collections::HashSet;
use std::ops::Sub;
use std::str::FromStr;
use std::time::Instant;

struct Card {
    winning_numbers: HashSet<u64>,
    present_numbers: HashSet<u64>
}

fn parse_input(input: &str) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();
    for line in input.lines() {
        let splits: Vec<&str> = line.split_terminator(&[':', '|'][..]).collect();
        if splits.len() != 3 {
            eprintln!("Line: {} is not formatted correctly!", line);
            continue
        }

        let winning_numbers: HashSet<u64> = (splits[1]).split_whitespace().map(|x| u64::from_str(x).unwrap()).collect();
        let present_numbers: HashSet<u64> = (splits[2]).split_whitespace().map(|x| u64::from_str(x).unwrap()).collect();
        cards.push(Card { winning_numbers, present_numbers});
    }
    cards
}

fn calculate_card_value(card: &Card) -> u64 {
    let correct_winning_numbers_count = card.present_numbers.intersection(&card.winning_numbers).count();
    if correct_winning_numbers_count == 0 {
        return 0
    }
    u64::pow(2, u32::sub(correct_winning_numbers_count as u32, 1_u32))
}

fn calculate_result(cards: Vec<Card>) -> u64 {
    cards.iter().map(calculate_card_value).sum::<u64>()
}


fn main() {
    let now = Instant::now();
    let input = include_str!("../../resources/input_01.txt");
    let cards = parse_input(input);
    let res = calculate_result(cards);
    println!("{}", res);
    let elapsed = now.elapsed();
    println!("Elapsed: {:?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input01() {
        let input = include_str!("../../resources/test_input_01.txt");
        let cards = parse_input(input);
        let res = calculate_result(cards);
        assert_eq!(res, 13);
    }
}
