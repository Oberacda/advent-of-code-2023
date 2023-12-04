use std::collections::HashSet;
use std::str::FromStr;

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


fn calculate_card_value(card: &Card) -> usize {
    card.present_numbers.intersection(&card.winning_numbers).count()
}

fn calculate_result(cards: Vec<Card>) -> u64 {
    let no_of_cards: usize = cards.len();
    let card_wins: Vec<u64> = cards.iter().map(|card| calculate_card_value(card) as u64).collect();
    let mut card_count: Vec<u64> = vec![1; no_of_cards];

    let mut result: u64 = 0;

    for idx in 0..no_of_cards {
        let wins = card_wins[idx];
        let count = card_count[idx];

        for idy in & mut card_count [idx+1..idx+ (wins as usize) + 1] {
            *idy += count;
        }
        result += count;
    }
    result
}

fn main() {

    let input = include_str!("../../resources/input_01.txt");
    let cards = parse_input(input);
    let res = calculate_result(cards);
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input01() {
        let input = include_str!("../../resources/test_input_01.txt");
        let cards = parse_input(input);
        let res = calculate_result(cards);
        assert_eq!(res, 30);
    }
}
